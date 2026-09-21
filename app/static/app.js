"use strict";

const state = {
  ownerId: "owner-001",
  businessId: "business-001",
  status: null,
  consent: null,
  package: null,
  response: null,
};

const $ = (selector, root = document) => root.querySelector(selector);
const $$ = (selector, root = document) => [...root.querySelectorAll(selector)];

function commandKey(prefix) {
  return `${prefix}-${crypto.randomUUID()}`;
}

async function api(path, { method = "GET", actor = state.ownerId, body, key } = {}) {
  const headers = { "X-Forhemit-Actor-ID": actor };
  if (body !== undefined) {
    headers["Content-Type"] = "application/json";
    headers["Idempotency-Key"] = key || commandKey("ui");
  }
  const response = await fetch(path, {
    method,
    headers,
    body: body === undefined ? undefined : JSON.stringify(body),
    cache: "no-store",
  });
  const payload = await response.json();
  if (!response.ok) {
    const error = new Error(payload.error?.message || `Request failed (${response.status})`);
    error.code = payload.error?.code;
    throw error;
  }
  return payload.data;
}

function showNotice(message, isError = false) {
  const notice = $("#notice");
  notice.textContent = message;
  notice.classList.toggle("error", isError);
  notice.hidden = false;
}

function clearNotice() {
  $("#notice").hidden = true;
}

function setBusy(form, busy) {
  $$("button", form).forEach((button) => {
    button.disabled = busy;
  });
}

async function runForm(form, action) {
  clearNotice();
  setBusy(form, true);
  try {
    await action(new FormData(form));
  } catch (error) {
    showNotice(`${error.code ? `${error.code}: ` : ""}${error.message}`, true);
  } finally {
    setBusy(form, false);
  }
}

function refsByStage() {
  return Object.fromEntries(
    (state.status?.stages || []).map((stage) => [stage.stage, stage.object_reference])
  );
}

function updateStatusView() {
  if (!state.status) return;
  $("#next-action strong").textContent = state.status.next_action;

  let foundCurrent = false;
  for (const stage of state.status.stages) {
    const nav = $(`[data-stage="${stage.stage}"]`);
    const panel = $(`[data-panel="${stage.stage}"]`);
    const complete = stage.status === "complete";
    const inProgress = stage.status === "in_progress";
    nav.classList.toggle("complete", complete);
    panel.classList.toggle("complete", complete);
    panel.classList.toggle("in-progress", inProgress);
    $(".status-pill", panel).textContent = complete
      ? "Complete"
      : inProgress
        ? "In progress"
        : "Not started";
    if (!complete && !foundCurrent) {
      nav.classList.add("active");
      foundCurrent = true;
    } else {
      nav.classList.remove("active");
    }
  }

  const finalStage = state.status.stages.at(-1);
  const determinationRef = finalStage?.object_reference;
  if (determinationRef && finalStage.status === "in_progress") {
    state.response = state.response || { response_id: determinationRef.object_id };
    $("#determination-card").hidden = false;
  }
  const complete = state.status.stages.every((stage) => stage.status === "complete");
  $("#completion-card").hidden = !complete;
  if (complete) $("#determination-card").hidden = true;
}

async function loadStatus({ quiet = false } = {}) {
  state.ownerId = $("#owner-id").value.trim();
  state.businessId = $("#business-id").value.trim();
  if (!state.ownerId || !state.businessId) {
    throw new Error("Owner ID and business ID are required.");
  }
  state.status = await api(
    `/v1/owners/${encodeURIComponent(state.ownerId)}/businesses/${encodeURIComponent(state.businessId)}/status`
  );
  updateStatusView();
  if (!quiet) showNotice("Local workspace loaded.");
  return state.status;
}

function objectRef(stage) {
  const value = refsByStage()[stage];
  if (!value) throw new Error(`Complete ${stage.replaceAll("_", " ")} first.`);
  return value;
}

function factsFrom(form) {
  const now = new Date().toISOString();
  const source = {
    source_id: "owner-ui-entry",
    source_type: "owner_input",
    source_version: "1",
    information_status: "stated",
    observed_at: now,
    excerpt_hash: null,
  };
  const fact = (index, field, value, valueStatus = "known") => ({
    fact_id: `fact-${String(index).padStart(3, "0")}`,
    field,
    value,
    value_status: valueStatus,
    source_authority: "owner_stated",
    sensitivity: "restricted",
    provenance: [{ ...source }],
  });

  return [
    fact(1, "business_name", form.get("business_name")),
    fact(2, "entity_type", form.get("entity_type")),
    fact(3, "revenue_range", {
      min: Number(form.get("revenue_min")),
      max: Number(form.get("revenue_max")),
    }, "range"),
    fact(4, "operating_cash_flow_range", {
      min: Number(form.get("cashflow_min")),
      max: Number(form.get("cashflow_max")),
    }, "range"),
    fact(5, "employee_count", Number(form.get("employee_count"))),
    fact(6, "ownership_summary", form.get("ownership_summary")),
    fact(7, "management_summary", form.get("management_summary")),
    fact(8, "debt_range", null, "unknown"),
  ];
}

$("#identity-form").addEventListener("submit", (event) => {
  event.preventDefault();
  runForm(event.currentTarget, () => loadStatus());
});

$("#destination-form").addEventListener("submit", (event) => {
  event.preventDefault();
  runForm(event.currentTarget, async (form) => {
    await api("/v1/destinations/confirm", {
      method: "POST",
      body: {
        owner_id: state.ownerId,
        business_id: state.businessId,
        destination_id: `${state.businessId}-destination`,
        transition_horizon: form.get("transition_horizon"),
        employee_ownership_intent: form.get("employee_ownership_intent"),
        owner_involvement: form.get("owner_involvement"),
        objectives: [{
          objective_id: `${state.businessId}-objective-1`,
          statement: form.get("objective"),
          priority: 1,
        }],
        nonnegotiables: [{
          nonnegotiable_id: `${state.businessId}-nonnegotiable-1`,
          statement: form.get("nonnegotiable"),
          source_authority: "owner_stated",
        }],
      },
      key: commandKey("destination"),
    });
    await loadStatus({ quiet: true });
    showNotice("Owner intent confirmed and versioned.");
  });
});

$("#snapshot-form").addEventListener("submit", (event) => {
  event.preventDefault();
  runForm(event.currentTarget, async (form) => {
    await api("/v1/business-snapshots/confirm", {
      method: "POST",
      body: {
        owner_id: state.ownerId,
        business_id: state.businessId,
        current_state_id: `${state.businessId}-snapshot`,
        primary_jurisdiction: "US-CA",
        facts: factsFrom(form),
      },
      key: commandKey("snapshot"),
    });
    await loadStatus({ quiet: true });
    showNotice("Business snapshot confirmed. Unknown debt remains unknown.");
  });
});

$("#scenario-form").addEventListener("submit", (event) => {
  event.preventDefault();
  runForm(event.currentTarget, async (form) => {
    await api("/v1/scenarios", {
      method: "POST",
      body: {
        owner_id: state.ownerId,
        scenario_id: `${state.businessId}-scenario`,
        destination_reference: objectRef("destination"),
        current_state_reference: objectRef("business_snapshot"),
        assumptions: [form.get("assumption")],
        unknowns: [form.get("unknown")],
        conflicts: [],
        professional_questions: [form.get("question")],
      },
      key: commandKey("scenario"),
    });
    await loadStatus({ quiet: true });
    showNotice("Exploratory scenario created. No feasibility determination was made.");
  });
});

$("#package-form").addEventListener("submit", (event) => {
  event.preventDefault();
  runForm(event.currentTarget, async (form) => {
    const scenario = objectRef("esop_scenario_exploration");
    const days = Number(form.get("duration_days"));
    const expires = new Date(Date.now() + days * 86_400_000).toISOString();
    const consentId = `${state.businessId}-professional-consent`;
    state.consent = await api("/v1/professional-consents", {
      method: "POST",
      body: {
        owner_id: state.ownerId,
        consent_id: consentId,
        recipient_id: form.get("professional_id"),
        purpose: "california_esop_exploration_review",
        resource_references: [scenario],
        permissions: ["full_view"],
        expires_at: expires,
      },
      key: commandKey("consent"),
    });

    state.package = await api("/v1/review-packages/authorize", {
      method: "POST",
      body: {
        owner_id: state.ownerId,
        business_id: state.businessId,
        package_id: `${state.businessId}-review-package`,
        recipient_id: form.get("professional_id"),
        professional_role: form.get("professional_role"),
        purpose: "california_esop_exploration_review",
        destination_reference: objectRef("destination"),
        current_state_reference: objectRef("business_snapshot"),
        scenario_reference: scenario,
        consent_reference: {
          object_type: "ConsentRecord",
          object_id: state.consent.consent_id,
          version: state.consent.version,
        },
        included_categories: [
          "destination",
          "business_snapshot",
          "exploratory_scenario",
          "professional_questions",
        ],
        excluded_categories: ["raw_documents", "external_ai_records"],
      },
      key: commandKey("package"),
    });

    await api("/v1/dev/workers/deliver-packages", {
      method: "POST",
      actor: "local-system",
      body: {},
      key: commandKey("deliver"),
    });
    await loadStatus({ quiet: true });
    showNotice("Consent and disclosure recorded; the minimized package was delivered.");
  });
});

$("#revoke-consent").addEventListener("click", () => {
  const form = $("#package-form");
  runForm(form, async () => {
    const consentId = state.consent?.consent_id ||
      `${state.businessId}-professional-consent`;
    await api(`/v1/professional-consents/${encodeURIComponent(consentId)}/revoke`, {
      method: "POST",
      body: { owner_id: state.ownerId },
      key: commandKey("revoke"),
    });
    state.consent = null;
    showNotice("Professional-sharing consent revoked. Pending delivery is blocked.");
  });
});

$("#professional-form").addEventListener("submit", (event) => {
  event.preventDefault();
  runForm(event.currentTarget, async (form) => {
    const packageRef = objectRef("professional_review_package");
    state.response = await api(
      `/v1/review-packages/${encodeURIComponent(packageRef.object_id)}/versions/${packageRef.version}/responses`,
      {
        method: "POST",
        actor: form.get("professional_id"),
        body: {
          professional_role: form.get("professional_role"),
          response_text: form.get("response_text"),
          category: "information_request",
        },
        key: commandKey("professional-response"),
      }
    );
    await api("/v1/dev/workers/import-responses", {
      method: "POST",
      actor: "local-system",
      body: {},
      key: commandKey("import"),
    });
    $("#determination-summary").textContent = form.get("response_text");
    $("#determination-card").hidden = false;
    await loadStatus({ quiet: true });
    showNotice("Attributed professional feedback imported into the owner workspace.");
  });
});

$("#acknowledgment-form").addEventListener("submit", (event) => {
  event.preventDefault();
  runForm(event.currentTarget, async () => {
    const determinationId = state.response?.response_id ||
      state.status?.stages.at(-1)?.object_reference?.object_id;
    if (!determinationId) throw new Error("No attributed response is available.");
    await api(
      `/v1/professional-determinations/${encodeURIComponent(determinationId)}/acknowledgments`,
      {
        method: "POST",
        body: {
          owner_id: state.ownerId,
          statement: "I acknowledge receipt without selecting or initiating a transaction.",
        },
        key: commandKey("acknowledgment"),
      }
    );
    await loadStatus({ quiet: true });
    showNotice("Acknowledgment recorded. No agreement or transaction handoff was created.");
  });
});

loadStatus({ quiet: true }).catch((error) => {
  showNotice(`Ready for a new local workflow. ${error.message}`, false);
});