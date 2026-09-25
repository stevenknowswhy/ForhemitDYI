// Wire-name vocabularies for the scenario explorer — the snake_case enum
// names the shell decodes into contract enums, each with the human label
// used on screen. Presentation data only; the engine remains the
// validator of every value.

export interface VocabOption {
  value: string;
  label: string;
}

export const SCENARIO_TYPES: VocabOption[] = [
  { value: "esop", label: "ESOP — employee stock ownership plan" },
  { value: "direct_employee_purchase", label: "Direct employee purchase" },
  { value: "management_buyout", label: "Management buyout" },
  { value: "employee_owned_acquisition_entity", label: "Employee-owned acquisition entity" },
  { value: "staged_ownership", label: "Staged ownership transfer" },
  { value: "seller_financed_employee_acquisition", label: "Seller-financed employee acquisition" },
  { value: "hybrid", label: "Hybrid of structures" },
  { value: "retain_and_transition", label: "Retain and transition gradually" },
  { value: "other", label: "Other" },
];

export const ASSUMPTION_CATEGORIES: VocabOption[] = [
  { value: "business", label: "The business itself" },
  { value: "financial", label: "Financial figures and models" },
  { value: "ownership", label: "Ownership structure and percentages" },
  { value: "market", label: "Market conditions" },
  { value: "financing", label: "Deal financing" },
  { value: "tax", label: "Tax effects" },
  { value: "legal", label: "Legal and regulatory" },
  { value: "operational", label: "Operating realities" },
  { value: "timing", label: "Timing of the transition" },
  { value: "employee", label: "Employees and culture" },
  { value: "governance", label: "Governance" },
  { value: "seller", label: "The seller's position" },
  { value: "buyer", label: "The buyers' position" },
  { value: "external_environment", label: "The world outside the deal" },
  { value: "other", label: "Other" },
];

export const PROVENANCES: VocabOption[] = [
  { value: "owner_reported", label: "Owner reported — you said this" },
  { value: "document_supported", label: "Document supported" },
  { value: "research_supported", label: "Research supported" },
  { value: "professionally_supplied", label: "Professionally supplied" },
  { value: "scenario_assumed", label: "Scenario assumed — for exploring" },
  { value: "system_derived", label: "System derived" },
  { value: "model_derived", label: "Model derived" },
];

export const VERIFICATIONS: VocabOption[] = [
  { value: "unknown", label: "Unknown" },
  { value: "unverified", label: "Unverified" },
  { value: "partially_verified", label: "Partially verified" },
  { value: "verified", label: "Verified" },
  { value: "professionally_verified", label: "Professionally verified" },
  { value: "contested", label: "Contested" },
  { value: "rejected", label: "Rejected" },
];

export const UNKNOWN_IMPORTANCES: VocabOption[] = [
  { value: "critical", label: "Critical — cannot be used for its purpose without it" },
  { value: "important", label: "Important — materially changes the picture" },
  { value: "helpful", label: "Helpful — nice to know" },
];

export const UNKNOWN_STATUSES: VocabOption[] = [
  { value: "open", label: "Open" },
  { value: "in_progress", label: "In progress" },
  { value: "resolved", label: "Resolved" },
  { value: "waived", label: "Waived — proceeding without it" },
  { value: "superseded", label: "Superseded" },
];

export const CONSTRAINT_TYPES: VocabOption[] = [
  { value: "hard", label: "Hard boundary — the scenario must respect it" },
  { value: "strong_preference", label: "Strong preference — override only with a recorded decision" },
  { value: "preference", label: "Preference" },
  { value: "unknown", label: "Strength unknown" },
  { value: "professional_requirement", label: "Set by a professional" },
  { value: "external_constraint", label: "Set by an outside party" },
];

export const CONFLICT_TYPES: VocabOption[] = [
  { value: "owner_objective", label: "Contradicts an owner objective" },
  { value: "nonnegotiable", label: "Conflicts with a destination nonnegotiable" },
  { value: "data", label: "Data contradicts itself or the record" },
  { value: "research", label: "Contradicts research" },
  { value: "professional", label: "Contradicts a professional's determination" },
  { value: "model", label: "Contradicts a model output" },
  { value: "dependency", label: "Depends on something unavailable" },
  { value: "other", label: "Other" },
];

export const CONFLICT_SEVERITIES: VocabOption[] = [
  { value: "informational", label: "Informational — worth knowing, nothing more" },
  { value: "attention", label: "Attention — you should look at this" },
  { value: "material", label: "Material — this changes the scenario's meaning" },
  { value: "blocking", label: "Blocking — this path cannot proceed until resolved" },
];

export const BRANCH_TYPES: VocabOption[] = [
  { value: "what_if", label: "What if…?" },
  { value: "alternative", label: "A distinct alternative structure" },
  { value: "stress_case", label: "A deliberate worst-case probe" },
  { value: "professional_request", label: "Requested by a professional" },
  { value: "owner_request", label: "Requested by the owner" },
];

export const DIMENSION_TYPES: VocabOption[] = [
  { value: "owner_objective", label: "Owner objective" },
  { value: "financial", label: "Financial" },
  { value: "ownership", label: "Ownership structure" },
  { value: "personal", label: "Personal circumstances" },
  { value: "business", label: "Business continuity" },
  { value: "timing", label: "Timing" },
  { value: "financing", label: "Financing" },
  { value: "seller_note", label: "Seller-note terms" },
  { value: "uncertainty", label: "Uncertainty" },
  { value: "information_completeness", label: "Information completeness" },
  { value: "evidence", label: "Evidence quality" },
];

// The outcome words are factual statuses — never ranks or scores
// (schema doc §33 "comparison without winner").
export const COMPARISON_OUTCOMES: VocabOption[] = [
  { value: "aligns", label: "Aligns" },
  { value: "does_not_currently_align", label: "Does not currently align" },
  { value: "insufficient_information", label: "Not enough information to say" },
];

export const OWNER_DECISIONS: VocabOption[] = [
  {
    value: "keep_requirement",
    label: "Keep the must-have — the scenario changes or is abandoned",
  },
  { value: "explore_another_path", label: "Explore a different path" },
  {
    value: "change_requirement",
    label: "Change the requirement — edit the destination (a new version)",
  },
];

export const READINESS_LEVELS: VocabOption[] = [
  { value: "preliminary", label: "Preliminary — early, honestly so" },
  { value: "information_needed", label: "Information needed — named gaps exist" },
  { value: "modelable", label: "Modelable — structured enough to compute over" },
  { value: "ready_for_comparison", label: "Ready for comparison" },
  { value: "ready_for_professional_review", label: "Ready for the professional review package" },
  { value: "under_review", label: "Under professional review" },
  { value: "revised", label: "Revised after review" },
  { value: "superseded", label: "Superseded" },
  { value: "archived", label: "Archived" },
];

export const VALUE_TYPES: VocabOption[] = [
  { value: "number", label: "Number" },
  { value: "currency", label: "Currency" },
  { value: "percentage", label: "Percentage" },
  { value: "duration_months", label: "Duration (whole months)" },
  { value: "boolean", label: "Yes / no" },
  { value: "text", label: "Text" },
];

export const LIFECYCLE_LABELS: Record<string, string> = {
  idea: "Idea",
  draft: "Draft",
  preliminary: "Preliminary",
  modelable: "Modelable",
  comparable: "Comparable",
  under_professional_review: "Under professional review",
  revised: "Revised",
  finalized: "Finalized",
  superseded: "Superseded",
  archived: "Archived",
};

export const SEVERITY_CLASSES: Record<string, string> = {
  informational: "sev-informational",
  attention: "sev-attention",
  material: "sev-material",
  blocking: "sev-blocking",
};

export function vocabLabel(options: VocabOption[], value: string): string {
  return options.find((option) => option.value === value)?.label ?? value;
}

export function readinessLabel(value: string): string {
  return vocabLabel(READINESS_LEVELS, value);
}

export function outcomeLabel(value: string): string {
  return vocabLabel(COMPARISON_OUTCOMES, value);
}

export function lifecycleLabel(value: string): string {
  return LIFECYCLE_LABELS[value] ?? value;
}
