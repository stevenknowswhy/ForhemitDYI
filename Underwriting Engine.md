The financing conversation does not fail because the business is weak. It fails because the file is incomplete, the numbers disagree, and the owner is asking a lender to assemble their own case.

An owner should never have to say, "I'll get that to you." This engine exists so that when the packet is handed over, nothing is missing, nothing contradicts anything else, and every figure can be traced to a source the lender can inspect.

The engine assembles the packet. It does not decide whether the loan is good.

# Underwriting Engine

## 1. Purpose

The Underwriting Engine generates a complete, lender-ready underwriting packet for a business financing request.

The packet is designed to be handed directly to:

* a commercial bank
* an SBA-participating lender
* a private investor or equity provider
* a seller-note holder or mezzanine provider

It must contain everything those parties need to underwrite the request, so that the deal proceeds with minimal friction, minimal back-and-forth, and no round trips caused by missing or contradictory paperwork.

The engine answers one question:

> **What does this specific lender require in order to say yes or no, and is every one of those things present, current, consistent, and traceable?**

It does not answer:

> **Should this loan be approved?**

That is the lender's judgment. The engine's job ends at making the judgment possible without delay.

---

# 2. The Core Architectural Principle

> **The engine owns conformance to an external standard. It does not own the facts, the numbers, or the decision.**

This is what separates it from every other engine in the platform. Every other engine models the platform's own view of the owner's situation. This engine models **someone else's requirements** — a lender's underwriting standard — and tests the owner's file against it.

The practical consequences:

* The engine never invents a requirement. Requirements are declared data, versioned, and sourced.
* The engine never computes a financial result it does not own. It requests the number from the engine that owns it.
* The engine never decides creditworthiness. It reports conformance and lets the lender decide.
* The engine never hides a gap. An incomplete packet is reported as incomplete, with the specific missing items named.

The engine is a **conformance engine**, not an advisory engine.

---

# 3. Why This Engine Exists

Financing conversations fail for boring, preventable reasons. Every one of them is an assembly problem, not a credit problem.

## 3.1 The file is incomplete

Lenders stop at an incomplete file. A missing signature on a personal financial statement, or one owner's tax return out of three, stalls review for weeks while everyone waits.

## 3.2 The numbers disagree

Revenue on the tax return does not match revenue on the profit and loss statement, which does not match deposits in the bank statements. The lender cannot tell whether the business is inconsistent or the paperwork is. Either way, underwriting stops and asks.

## 3.3 The financials are stale

A profit and loss statement dated four months ago is not evidence of current performance. Most lenders require interim financials dated within 90 days of submission. Stale financials trigger an automatic request for updated versions.

## 3.4 The owner is asking the lender to build the case

If the lender has to ask "what is the loan for?", "what is the ownership structure?", "what is being pledged?", the owner has handed over a pile of documents instead of a case.

## 3.5 Nothing records what was actually sent

When a lender says "we never received the appraisal," or asks for the version of the projections that was submitted in March, there is no answer.

The Underwriting Engine exists to make all five failure modes structurally impossible.

---

# 4. What This Engine Owns

* The **lender requirement model** — what a given program or institution requires
* **Requirement profiles** for SBA programs, conventional commercial lending, and investor due diligence
* The **requirement-to-source mapping** — which artifact satisfies which requirement
* **Packet structure** — the sections, their order, and what each contains
* **Packet assembly** — collecting, deriving, ordering, and numbering every item
* **Derived schedules** that are purely presentational roll-ups: the business debt schedule, the source and use of funds statement, the collateral schedule, the ownership and structure chart
* **Consistency checking across documents** — reporting where figures disagree
* The **reconciliation memo** that explains a known difference
* **Completeness gating** — the submittable / not-submittable determination
* **Staleness rules** — whether an item is too old to submit
* **Packet versioning and reproduction** — the exact content of what was sent, to whom, when
* The **submission record**
* The **follow-up log** — what the lender asked for after submission, and whether it has been answered
* The **cover memo** and table of contents
* File naming and organization conventions

---

# 5. What This Engine Does NOT Own

It does not own:

* Business facts, financial statements, or the current-state assessment — **Business Reality**
* Debt service coverage, global cash flow, affordability, or any financial calculation — **Financial Modeling**
* The value of the business or the reasonableness of the price — **Valuation**
* Which financing structures exist, which lenders to approach, or how to structure the deal — **Capital / Financing**
* Whether a document exists, is complete, or is ready — **Document Readiness**
* Generic stakeholder package assembly — **Review Package**
* Professional determinations of any kind — **Professional Review**
* Source provenance and evidence authority — **Evidence Ledger**
* Conflicts between sources — **Fact Verification**
* Who may see which document, and for what purpose — **Consent & Access**
* Document storage, encryption, and versioning of the underlying files — **Local Vault**
* The record of what happened — **Audit / Provenance**
* The credit decision

> **The engine owns the submission. It does not own the substance being submitted.**

---

# 6. What a Lender Actually Does With the Packet

Understanding the lender's process is what makes the packet correct.

```text
1. ELIGIBILITY
   Is the borrower, the use of proceeds, and the business type eligible?

2. DOCUMENT VERIFICATION
   Are the required documents present, signed, and current?

3. SPREADING AND CASH FLOW
   Are the figures extracted, normalized, and consistent?
   What is the debt service coverage, historically and projected?

4. CREDIT ASSESSMENT
   Credit history, character, prior defaults, government financing history.

5. COLLATERAL AND SECURITY
   What is pledged, what is it worth, what liens exist?

6. EQUITY INJECTION
   How much of the owner's own money is in the deal, from where, seasoned how?

7. GLOBAL CASH FLOW
   Does the guarantor's personal position support the obligation?

8. CREDIT MEMO AND DECISION
   An underwriter writes the case and decides.
```

The packet must serve steps 1 through 7. Step 8 is the lender's alone.

> **The packet's job is to make step 8 a reading exercise rather than an investigation.**

---

# 7. The Three Lender Profiles

The engine does not produce one packet. It produces the packet a **specific** audience requires.

| Profile | Primary test | Packet character |
| --- | --- | --- |
| **SBA-participating lender** | Eligibility, repayment ability, credit, injection | Highly prescriptive. Government forms, mandated disclosures, program-specific rules. |
| **Conventional commercial bank** | Repayment ability, collateral coverage, relationship | Less prescriptive on forms, more judgment on credit. Bank's own template governs. |
| **Investor / equity provider** | Earnings quality, risk, growth, defensibility | Investigative. Due diligence across the whole business, not just the financials. |

A single transaction may need more than one packet. The engine supports multiple concurrent profiles against the same underlying facts.

> **Same facts, different conformance standards. The facts are assembled once; the packets are rendered per audience.**

---

# 8. The SBA Profile

The SBA profile is the most prescriptive, and the one where an incomplete file is most costly.

## 8.1 Program-specific forms

| Form | Name | Purpose |
| --- | --- | --- |
| **SBA Form 1919** | Borrower Information Form | Applicant and owner disclosure, loan request, existing indebtedness, prior government financing |
| **SBA Form 912** | Statement of Personal History | Background certification for each owner of 20% or more |
| **SBA Form 413** | Personal Financial Statement | Personal balance sheet, net worth, and liquid assets for each principal |
| **SBA Form 4** | Application for Guaranty | The application itself, completed by the lender |
| **SBA Form 159** | Fee Disclosure | Compensation to agents and packagers |
| **SBA Form 601** | Agreement of Compliance | Required where real estate is involved |
| **SBA Form 1050 / 1050A** | Franchise disclosure | Where the business is a franchise |
| **7(a) Loan Submission Checklist Cover Sheet** | Submission cover | The lender's own submission checklist |

## 8.2 Business financial requirements

* **Business tax returns — three years**, complete and unredacted, including all schedules, K-1s, depreciation schedules, and balance-sheet pages. Not front pages only.
* **Interim profit and loss statement** — dated within 90 days of submission, showing the current fiscal year to date.
* **Interim balance sheet** — dated within 90 days of submission.
* **Business bank statements** — the two most recent months of the primary operating account are required to confirm existing debts and obligations. Three to twelve months are commonly requested to read the trend.
* **Business debt schedule** — every creditor, original amount, current balance, interest rate, monthly payment, maturity, and collateral.
* **Two-year financial projections** — monthly for the first twelve months, annual for the second year, with stated assumptions.

## 8.3 Personal and guarantor requirements

* **Personal tax returns — three years** for every owner of 20% or more.
* **SBA Form 413** for each such owner.
* **Driver's license or state identification**, front and back.
* **Permanent Resident Card and USCIS documentation** for non-citizen owners, with Form I-9 where applicable.
* **Personal financial statement** and supporting schedules.
* **Equity injection documentation** — source, amount, seasoning, and evidence.

## 8.4 Legal and organizational requirements

* Articles of Organization or Incorporation, filed with the Secretary of State
* Operating Agreement, Bylaws, or Partnership Agreement
* Business licenses and permits, current
* Commercial lease or property deed
* EIN confirmation letter (CP 575)
* Any DBA filings

## 8.5 Collateral and property requirements

* Titles, UCC filings, appraisals, and purchase agreements
* Environmental assessments where required by collateral type or industry
* Insurance certificates and evidence of coverage
* Property deed, purchase and sale contract, existing lien information, property tax receipts, leases, and photographs for real-estate-secured requests

## 8.6 Acquisition-specific requirements

Where the loan funds a business acquisition, the packet additionally requires:

* Letter of intent
* Signed purchase agreement
* **Seller's three years of financial statements**
* Accounts receivable and accounts payable aging
* Certified business valuation
* Business plan for the acquired entity

## 8.7 The underwriting tests the packet must support

The packet must contain enough to let the lender compute and defend:

* **Debt service coverage** — at or above the applicable program floor, on a historical and/or projected basis
* **Global debt service coverage** — folding in the guarantors' personal obligations, required for every owner of 20% or more
* **Credit standing** — personal and business credit
* **Equity injection** — typically around ten percent on acquisitions and startups, from the borrower's own funds, verified, and not itself borrowed
* **Collateral position**
* **Time in business** and, on a change of ownership, the seller's operating record

> **These thresholds are program parameters, not engine constants. See section 11.**

---

# 9. The Conventional Bank Profile

A conventional commercial bank is less prescriptive about forms and more dependent on its own credit policy. The packet still must be complete, but the bank's template governs the ordering.

## 9.1 Core requirements

* Basic personal information for each principal — legal name, prior names, address, identification
* Basic business information — operating address, entity type, EIN
* Business licenses and permits
* **Two years of business and personal tax returns**
* **Recent business bank statements** — three to four months minimum
* Most recent profit and loss statement
* Cash flow forecast
* Business debt schedule
* Statement of the use of loan proceeds, specific enough to match the amount requested
* Collateral documentation
* Business plan, where the business is new or the request is for growth capital

## 9.2 Real-estate-secured additions

* Information on all parties — borrowing entity, management company, current owners
* Property deed
* Purchase and sale contract
* Existing lien information
* Property tax receipts
* Leases and rental agreements
* Property photographs
* Proof of required insurance

## 9.3 What differs from the SBA profile

* No government forms
* The bank's own credit memo template governs structure
* Collateral coverage and relationship history carry more weight than a program floor
* The bank may accept fewer years of tax returns for a long-standing customer
* Personal guarantees are a matter of bank policy rather than program rule

---

# 10. The Investor Profile

An investor is not underwriting a loan. They are investigating a business. The packet is a **due diligence file**, and it is far broader than a loan file.

The investor profile is organized into eight workstreams, each of which is a section of the packet.

## 10.1 Financial

* Audited financial statements, three years, where they exist
* Unaudited monthly financials, twenty-four months
* Federal and state tax returns, three years
* **Quality of earnings analysis**
* Accounts receivable aging
* Accounts payable aging
* Revenue recognition policies
* Deferred revenue schedule
* Working capital analysis
* Capital expenditure history and projections
* Debt schedule with terms and covenants
* Bank statements, twelve months
* Inventory valuation and aging
* Fixed asset register
* Chart of accounts
* Intercompany transactions and balances
* Related party transactions
* Contingent liabilities and off-balance-sheet items
* Insurance policies and claims history
* Budget versus actual variance analysis
* Financial projections and assumptions
* Cash flow statements and projections
* Break-even analysis
* Gross margin analysis by product or service
* **EBITDA adjustments and add-backs**

## 10.2 Legal

* Certificate of incorporation and amendments
* Bylaws and operating agreement
* Good standing certificates, all jurisdictions
* Board and shareholder meeting minutes
* Capitalization table and equity agreements
* Stock option plans and grants
* Material contracts list
* Customer contracts, top twenty
* Supplier and vendor contracts, top ten
* Real estate and equipment leases
* Loan and credit agreements
* Guarantees and security interests
* Partnership and joint venture agreements
* Licensing agreements, inbound and outbound
* Distribution and reseller agreements
* Non-compete and non-solicitation agreements
* Pending and threatened litigation
* Litigation history, five years
* Regulatory correspondence and investigations
* Consent decrees and settlement agreements
* Change-of-control provisions
* Assignment and consent requirements
* Powers of attorney
* UCC filings and liens

## 10.3 Operational

* Organization chart
* Business process documentation
* Facilities list with square footage
* Equipment and machinery list
* Maintenance and repair records
* Production capacity analysis
* Quality control procedures
* Product and service delivery processes
* Supply chain overview
* Vendor concentration analysis
* Inventory management systems
* Logistics and distribution network
* Key operational metrics and KPIs
* Capacity utilization rates
* Backlog and order pipeline
* Product roadmap and development pipeline
* Manufacturing agreements
* Warranties and returns history
* Business continuity and disaster recovery plans
* Standard operating procedures

## 10.4 Human resources

* Employee census with titles and tenure
* Executive team biographies
* Employment agreements for key employees
* Compensation structure and salary ranges
* Bonus and incentive plans
* Commission structures
* Health and welfare benefit plans
* Retirement and pension plans
* Stock option and equity compensation plans
* Employee handbook and policies
* Severance policies and agreements
* Non-compete agreements
* Non-disclosure agreements
* Independent contractor agreements
* Contractor versus employee classification review
* Union agreements and labor relations
* OSHA compliance and safety records
* Workers' compensation claims history
* Employee turnover statistics
* Pending employment claims
* I-9 and work authorization compliance
* Key person risk assessment
* Succession planning
* Training and development programs

## 10.5 Technology and IT

* IT infrastructure overview
* Software and systems inventory
* Software license agreements
* Cloud service agreements
* SaaS subscriptions and contracts
* Data architecture and database documentation
* API integrations and dependencies
* Source code ownership and access
* Technology development roadmap
* Patent portfolio and pending applications
* Trademark and copyright registrations
* Trade secrets inventory
* IP assignment agreements
* Open source software usage
* Cybersecurity policies and procedures
* Data breach history
* Penetration testing results
* SOC 2 or security audit reports
* Data privacy policies
* IT disaster recovery plan
* System uptime and reliability metrics
* Technical debt assessment
* IT budget and spending history

## 10.6 Commercial

* Customer list with revenue by customer
* **Customer concentration analysis**
* Customer retention and churn rates
* Customer satisfaction scores
* Sales pipeline and forecasting
* Win/loss analysis
* Pricing strategy and history
* Discount policies and practices
* Sales compensation and incentives
* Marketing strategy and budget
* Brand and trademark assets
* Website analytics and traffic data
* Market size and growth analysis
* Competitive landscape analysis
* Market share estimates
* Industry trends and outlook
* Regulatory environment impact
* Geographic market coverage
* Channel partner relationships
* Customer reference calls

## 10.7 Environmental

* Environmental permits and licenses
* Environmental compliance history
* **Phase I Environmental Site Assessment**
* Phase II Assessment, where triggered
* Hazardous materials handling procedures
* Waste disposal practices and records
* Air and water discharge permits
* Underground storage tank records
* Asbestos and lead paint surveys
* Contamination remediation history
* EPA or state agency correspondence
* Environmental insurance policies
* Sustainability and ESG programs
* Carbon footprint and emissions data
* Environmental litigation history

## 10.8 Regulatory and compliance

* Business licenses and permits
* Professional licenses
* Industry-specific regulatory compliance
* Agency approvals
* Import and export licenses and compliance
* Government contracts and compliance
* Anti-corruption and FCPA compliance
* Anti-money laundering compliance
* Data privacy and protection compliance
* Sector-specific compliance regimes where applicable
* Regulatory examination history
* Consent orders or regulatory settlements

> **The investor profile is deliberately much larger than a loan profile. It is the difference between "will they repay me" and "should I own this."**

---

# 11. The Requirement Model

Requirements are **data**, never code.

This is the most important architectural decision in the engine, and it exists for the same reason the Policy Engine refuses to hard-code policy: underwriting standards change, and an engine that hard-codes them becomes wrong silently.

Consider how much moves:

* Program standard operating procedures are reissued on their own schedule, and a reissued procedure can restore, tighten, or relax criteria wholesale.
* Coverage floors, credit-screening rules, and sizing rules have all changed on separate effective dates from the same program.
* Equity injection expectations vary by acquisition, startup, and expansion.
* Individual lenders layer their own credit policy on top of any program floor.
* Investor due diligence scope varies by deal size, sector, and provider.

Therefore:

```text
LenderRequirementProfile
        │
        ├── program            (SBA 7(a) | SBA 504 | conventional | investor | custom)
        ├── institution        (specific lender or provider, optional)
        ├── version            (the profile's own version)
        ├── effective_date     (when this profile became authoritative)
        ├── source_reference   (where these requirements came from)
        └── requirements[]     (PacketRequirement)
```

Every requirement carries:

* what it is
* whether it is mandatory or conditional
* the condition that makes it apply
* how current it must be
* whether it requires a signature
* whether it requires notarization or certification
* which engine or party supplies it

> **A requirement profile that cannot cite its source is not usable. The engine refuses to gate a packet against an unsourced profile.**

---

# 12. Requirement Categories

Every requirement belongs to exactly one category. The category determines how it is satisfied and who is asked.

| Category | Satisfied by | Examples |
| --- | --- | --- |
| **Platform fact** | An owning engine supplies it | Revenue history, debt service coverage, valuation range |
| **Uploaded document** | The owner or a professional provides a file | Tax return, lease, insurance certificate |
| **Executed form** | A form rendered, signed, and returned | Personal financial statement, borrower information form |
| **Third-party report** | An external provider produces it | Appraisal, environmental assessment, quality of earnings |
| **Derived schedule** | The engine computes a presentation roll-up | Business debt schedule, source and use of funds |
| **Narrative** | The owner or the platform drafts it | Business plan, use of proceeds statement, reconciliation memo |
| **Attestation** | A person asserts something | No pending litigation, source of equity injection |

---

# 13. Requirement Satisfaction

A requirement is satisfied only when **all** of its conditions hold:

```text
satisfied = present
          AND current          (within its staleness window)
          AND complete         (all pages, all schedules, all owners)
          AND executed         (signed / notarized / certified where required)
          AND legible          (searchable, correctly oriented, not truncated)
          AND consistent       (agrees with the other documents in the packet)
          AND permitted        (the recipient is allowed to receive it)
```

Anything less is a **gap**, and every gap is named specifically:

> **Not "financial documents incomplete."**
> **"Personal tax returns for 2023 are missing for one of two owners of 20% or more."**

The specificity is the product. A vague gap report is the same as no gap report.

---

# 14. The Packet Structure

Every packet, regardless of profile, follows the same skeleton. The profile determines which sections are present and what each must contain.

```text
PACKET
  │
  ├── 1. Cover Memo
  ├── 2. Table of Contents
  │
  ├── 3. Transaction Summary and Request
  ├── 4. Use of Proceeds and Source of Funds
  │
  ├── 5. Business Financial History
  ├── 6. Interim Financial Statements
  ├── 7. Business Debt Schedule
  ├── 8. Financial Projections
  │
  ├── 9. Personal Financial Statements and Guarantor Strength
  ├── 10. Equity Injection Documentation
  │
  ├── 11. Business Plan and Narrative
  ├── 12. Legal and Organizational Documents
  ├── 13. Ownership and Structure
  ├── 14. Collateral and Security
  │
  ├── 15. Program-Specific Forms
  ├── 16. Environmental and Compliance
  ├── 17. Due Diligence Exhibits
  │
  └── 18. Appendix and Index
```

The ordering is deliberate: the reader learns **why** before **how**, and sees **current** before **historical**.

> **The first substantive section states what the owner is trying to accomplish. A lender who does not understand the purpose cannot evaluate the request.**

---

# 15. Section 3 — Transaction Summary and Request

The first thing the lender reads. It must answer, in one page:

* Who is borrowing
* What entity structure is borrowing
* How much is requested
* What the money is for
* Over what term, at what structure
* What is being acquired, built, or refinanced
* What the owner is putting in
* Who is guaranteeing
* What the requested decision is

Every figure here must reconcile to the sections that follow. A mismatch between the summary and the detail is the fastest way to lose an underwriter's confidence.

---

# 16. Section 4 — Use of Proceeds and Source of Funds

A two-column statement that must balance.

| Use of funds | Amount |
| --- | --- |
| Purchase price | |
| Working capital | |
| Equipment | |
| Closing costs and fees | |
| Debt refinance | |
| **Total uses** | |

| Source of funds | Amount |
| --- | --- |
| Senior lender | |
| Seller note | |
| Owner equity injection | |
| Investor equity | |
| **Total sources** | |

The engine enforces that **total uses equal total sources**. A packet that does not balance is not submittable.

Where an item is itself financed — a broker fee rolled into the loan, for example — the engine flags it, because lenders test whether the injection is genuinely the borrower's own funds or borrowed money wearing a different label.

---

# 17. Section 5 — Business Financial History

Contents:

* Three years of business tax returns, complete with all schedules
* Three years of financial statements, where prepared
* Year-over-year summary of revenue, gross margin, operating expenses, and net income
* Explanation of any material year-over-year movement
* The normalized earnings presentation, where the profile requires it

The engine does not compute these figures. It requests them from **Business Reality** and **Financial Modeling**, and presents them with their sources attached.

---

# 18. Section 6 — Interim Financial Statements

* Profit and loss statement, dated within the profile's staleness window
* Balance sheet, dated within the same window
* Accounts receivable and accounts payable aging, where required
* Comparison against the same period in the prior year

> **Interim statements are the proof that the business is still the business the tax returns describe.**

The engine tracks the reporting date of every interim statement and refuses to submit a packet whose interim financials have aged out.

---

# 19. Section 7 — Business Debt Schedule

A derived schedule. This is the single most important roll-up the engine owns, because it feeds directly into the lender's coverage calculation.

| Column | Meaning |
| --- | --- |
| Creditor | Who holds the obligation |
| Obligation type | Term loan, line of credit, lease, mortgage, equipment |
| Original amount | At origination |
| Current balance | As of the stated date |
| Interest rate | Contract rate |
| Monthly payment | Principal and interest |
| Maturity date | When it ends |
| Collateral | What secures it |
| Status | Current, delinquent, in forbearance |
| In the coverage calculation | Yes / no, with reason |

The engine assembles this from business facts and financing records. It does **not** compute the coverage ratio — it hands the schedule to **Financial Modeling**, which owns that arithmetic.

---

# 20. Section 8 — Financial Projections

* Monthly projections for the first twelve months
* Annual projections for the second year, and beyond where required
* Stated assumptions for every material line
* The bridge from current performance to projected performance
* The projected debt service coverage under the request

The projections must be **tied to the use of proceeds**. A projection that shows growth without connecting it to what the loan funds is the most common reason a lender asks for a rewrite.

> **Assumptions must be visible. A projection without stated assumptions cannot be underwritten.**

---

# 21. Section 9 — Personal Financial Statements and Guarantor Strength

For every guarantor, and for every owner meeting the profile's ownership threshold:

* Personal financial statement
* Personal tax returns for the required years
* Personal cash flow and global debt service position
* Existing personal obligations, including contingent liabilities and guarantees
* Identification
* Immigration documentation where applicable
* Credit explanation for any derogatory item

The global position is where most owners are surprised. A guarantor's existing personal obligations can change the answer even when the business itself covers the requested debt comfortably.

---

# 22. Section 10 — Equity Injection Documentation

* Amount injected
* Source of funds, specifically
* Evidence of the source — statements, sale proceeds, savings history
* Seasoning — how long the funds have been where they are
* Confirmation that the injection is not borrowed, where the profile requires it
* Whether any portion is in the form of a seller note, and how that is treated

This section exists because injection is one of the few places an owner can unintentionally misrepresent the deal. The engine records what was asserted, what was evidenced, and by whom.

---

# 23. Section 11 — Business Plan and Narrative

* Executive summary
* Use of proceeds, stated in plain language
* Market overview
* Management structure and relevant experience
* Operations narrative
* How the loan generates the cash flow to repay it
* Principal resumes

The engine assembles and organizes this. It does not write the owner's business plan, and it must not present generated narrative as the owner's own words.

---

# 24. Section 12 — Legal and Organizational Documents

* Formation documents
* Governing documents
* Good standing certificates
* Licenses and permits
* EIN confirmation
* Leases and deeds
* Material contracts, where the profile requires them

---

# 25. Section 13 — Ownership and Structure

A derived exhibit. The engine renders:

* The ownership chart, down to the required threshold
* The borrowing entity and its relationship to any affiliate
* Guarantor identification and ownership percentage
* Intercompany or affiliate relationships that affect the credit
* Any trust, holding company, or layered structure

Layered ownership is a common source of underwriting delay, because the lender must trace who actually owns and who actually guarantees. The engine traces it before the lender has to.

---

# 26. Section 14 — Collateral and Security

* A collateral schedule: item, type, value, valuation source and date, existing liens, proposed lien position
* Titles, UCC filings, appraisals, and purchase agreements
* Insurance certificates and coverage evidence
* For real estate: deed, purchase and sale contract, existing liens, tax receipts, leases, photographs

The engine assembles the schedule. It does **not** appraise anything, and it does not assert a value it cannot source.

---

# 27. Section 15 — Program-Specific Forms

Rendered from platform data where possible, then signed and returned.

The engine's role:

1. Determine which forms the profile requires
2. Pre-populate every field it can source from platform facts
3. Mark every field it cannot source, so the owner is not hunting for what is missing
4. Track signature and notarization state per form
5. Refuse to submit while any mandatory form is unexecuted

> **A missing signature stops review immediately. The engine treats an unsigned mandatory form as a hard block, not a warning.**

---

# 28. Section 16 — Environmental and Compliance

Where required by collateral type, industry, or profile:

* Environmental permits and compliance history
* Site assessments
* Regulatory approvals and licenses
* Sector-specific compliance evidence

The engine determines **whether** the requirement applies from the requirement profile and the business's industry and collateral — not from a guess.

---

# 29. Section 17 — Due Diligence Exhibits

Present only in the investor profile. This is where the eight due diligence workstreams from section 10 are rendered: financial, legal, operational, human resources, technology, commercial, environmental, and regulatory.

Each workstream is presented as:

* the item list
* the item's status — provided, outstanding, not applicable
* the reviewer assigned, where the transaction has one
* the finding, where review has occurred

---

# 30. Section 18 — Appendix and Index

* Full document index with page references
* The reconciliation memo
* The follow-up log from any prior submission
* A contact sheet for the owner's accountant, attorney, and other advisors, so the lender can resolve questions directly

> **A cover memo that names the accountant and attorney shortens underwriting. An underwriter who cannot ask a question will ask the borrower instead, and that costs a week.**

---

# 31. Data Inputs — Overview

The engine collects inputs from three places, and it must be explicit about which is which.

| Source | Meaning | Trust |
| --- | --- | --- |
| **Platform facts** | Supplied by an owning engine | Authoritative within that engine's domain |
| **Owner-provided** | Supplied by the owner or their advisors | Asserted until evidenced |
| **Third-party** | Supplied by an external provider | Authoritative for its own scope |

The engine never silently promotes an assertion to a fact. An asserted figure that has not been evidenced is labelled as asserted **in the packet**, because the lender will make that distinction whether or not the owner does.

---

# 32. Data Inputs — Business Financial

* Legal entity name, structure, formation date, jurisdictions
* EIN
* Fiscal year end
* Industry and NAICS classification
* Three years of financial statements and tax returns
* Interim profit and loss statement and balance sheet, with reporting date
* Bank statements for the required months
* Existing debt: creditor, balance, rate, payment, maturity, collateral, status
* Accounts receivable and payable aging
* Inventory, where material
* Fixed asset register
* Owner compensation and distributions
* Related-party transactions
* Existing leases and material contracts

---

# 33. Data Inputs — Guarantor and Personal

* Full legal name, prior names, identification
* Ownership percentage in the borrowing entity
* Home address and residency status
* Personal financial statement: assets, liabilities, net worth, liquid assets
* Personal tax returns for the required years
* Personal income sources
* Existing personal debt and monthly obligations
* Contingent liabilities and existing guarantees
* Equity injection: amount, source, evidence, seasoning
* Credit explanation for derogatory items
* Prior government financing history

---

# 34. Data Inputs — Legal and Organizational

* Formation and governing documents
* Good standing, all jurisdictions
* Ownership and capitalization, including the cap table where required
* Licenses and permits
* Leases and deeds
* Material contracts and change-of-control provisions
* Pending or threatened litigation
* Liens and UCC filings
* Insurance policies
* Intellectual property, where the profile requires it

---

# 35. Data Inputs — Collateral

* Item description and type
* Ownership of the item
* Valuation and its source
* Valuation date
* Existing liens and proposed lien position
* Insurance coverage
* For real property: legal description, deed, taxes, leases, occupancy

---

# 36. Data Inputs — Transaction and Program

* Requested amount
* Requested structure: term, amortization, rate basis where known
* Use of proceeds, itemized
* Source of funds, itemized
* Acquisition details: target entity, purchase agreement, letter of intent, seller financials
* Requested program
* Target institution, where identified
* Target submission date

> **The target submission date matters. It sets the staleness clock for every dated item in the packet.**

---

# 37. Data Inputs — Narrative

* Owner objectives and nonnegotiables
* Use of proceeds in plain language
* Business description and history
* Management background and relevant experience
* Market and competitive context
* How the financing produces repayment capacity
* Known weaknesses and how they are addressed

The engine may organize and format this material. It must not invent it, and it must not present platform-generated text as the owner's own statement of intent.

---

# 38. Data Inputs — Third-Party Reports

* Business valuation or appraisal
* Equipment appraisal
* Real property appraisal
* Environmental site assessment
* Quality of earnings report
* Survey
* Insurance certificates
* Accountant's compilation or review report
* Any professional determination relevant to the request

Each carries its provider, date, scope, and the requirement it satisfies.

---

# 39. The Assembly Pipeline

```text
1. PROGRAM SELECTION
        │
2. REQUIREMENT RESOLUTION      ← LenderRequirementProfile
        │
3. SOURCE RESOLUTION           ← which engine or party supplies each item
        │
4. DERIVATION                  ← build the roll-up schedules
        │
5. CONSISTENCY CHECKING        ← do the documents agree?
        │
6. COMPLETENESS GATING         ← is it submittable?
        │
7. ORDERING AND NUMBERING
        │
8. RENDERING
        │
9. DELIVERY AND RECORDING
```

Each step is idempotent and repeatable. Re-running assembly on unchanged inputs produces an identical packet.

---

# 40. Step 1 — Program Selection

The engine requires an explicit program before it can resolve requirements.

It does not infer the program from the deal size. It may **suggest** candidate programs based on the request, the use of proceeds, the entity, and the amount, and it must show why each was suggested.

> **A packet gated against the wrong program is worse than no packet, because it looks complete.**

---

# 41. Step 2 — Requirement Resolution

```text
for each program in selected_programs:
    profile = load_profile(program, institution, as_of_date)
    requirements = profile.requirements

    for each requirement:
        if requirement.conditional:
            if not condition_holds(requirement, transaction, business):
                mark not_applicable
                continue
        register(requirement)
```

The resolution records, for every requirement, **why it applies or does not**. A requirement marked not-applicable without a stated condition is a defect.

---

# 42. Step 3 — Source Resolution

For each requirement, determine the best available source.

```text
requirement: three years of business tax returns
    │
    ├── try: Local Vault for the document
    │       └── found 2022, 2023, 2024 → check currency, completeness, legibility
    │
    ├── if missing → request from owner
    │
    └── if partially present → gap names the specific missing year
```

Source resolution produces one of four outcomes:

* **Satisfied** — sourced, current, complete, executed
* **Gap** — a specific item is missing or deficient
* **Not applicable** — the condition does not hold, with reason
* **Blocked** — cannot be satisfied without an external action, with the action named

---

# 43. Step 4 — Derivation

The engine computes the schedules it owns:

* Business debt schedule, from debt records
* Source and use of funds, from the transaction
* Collateral schedule, from collateral records
* Ownership and structure chart, from entity and ownership records
* Document index, from the assembled items

Everything else is **requested**, not computed:

| Figure | Requested from |
| --- | --- |
| Debt service coverage | Financial Modeling |
| Global debt service coverage | Financial Modeling |
| Affordability and sensitivity | Financial Modeling |
| Valuation range and determination | Valuation |
| Current-state business facts | Business Reality |
| Source authority for a figure | Evidence Ledger |
| Conflicts between sources | Fact Verification |

> **The engine presents numbers. It does not author them.**

---

# 44. Step 5 — Consistency Checking

The step that prevents the most common cause of underwriting delay.

The engine checks the packet against itself and against its sources, and reports every disagreement. See section 48 for the check set.

A disagreement does not block submission by itself. An **unexplained** disagreement does.

---

# 45. Step 6 — Completeness Gating

```text
if any mandatory requirement is unsatisfied:
        status = NOT_SUBMITTABLE
        report every gap, specifically

elif any mandatory requirement is current but inconsistent,
     and the inconsistency is unexplained:
        status = NOT_SUBMITTABLE

elif any mandatory requirement is satisfied but stale:
        status = NOT_SUBMITTABLE

else:
        status = SUBMITTABLE
```

The gate is a hard gate. There is no override that silently ignores a gap.

A deliberate, recorded override may exist for a specific requirement, and it must carry:

* who authorized it
* why
* what the lender was told

> **The engine may permit a decision to submit with a known gap. It may never permit the gap to be invisible.**

---

# 46. Step 7 — Ordering and Numbering

Ordering is determined by the profile, not by file arrival.

Rules:

* Cover memo, then table of contents
* Purpose and request before detail
* Current before historical
* Business before personal
* Financial before legal
* Forms grouped together and separately tabbed
* Exhibits numbered, and referenced by number from the narrative
* Every item carries a stable item number, so a lender's question can be answered by number

---

# 47. Step 8 — Rendering

The engine renders the packet as an ordered, indexed, searchable document set.

Rendering rules:

* Every scanned page is legible, correctly oriented, and searchable
* Every form is rendered with its fields populated or explicitly marked
* Every derived schedule states its basis and its as-of date
* Every presented figure states its source
* The document index resolves every item to a page
* The packet renders identically on re-run

---

# 48. Step 9 — Delivery and Recording

Delivery produces an **UnderwritingSubmission**:

* the packet version
* the recipient
* the date and method
* the exact content — item list with hashes
* who sent it
* what was disclosed, under which consent

The submission record is what makes the answer to "which version did you send us?" possible months later.

---

# 49. Consistency Checks in Detail

| Check | What it compares | Why it matters |
| --- | --- | --- |
| **Revenue agreement** | Tax return vs. profit and loss vs. bank deposits | The single most common underwriting stop |
| **Balance sheet agreement** | Interim balance sheet vs. tax return balance sheet | Shows whether the books reconcile |
| **Debt agreement** | Debt schedule vs. bank statements vs. tax return interest expense | Reveals undisclosed obligations |
| **Use and source balance** | Total uses vs. total sources | A packet that does not balance is not submittable |
| **Request agreement** | Summary request vs. loan application vs. projections | A mismatch reads as carelessness |
| **Ownership agreement** | Ownership chart vs. formation documents vs. cap table | Reveals undisclosed owners and guarantors |
| **Guarantor coverage** | Guarantors named vs. owners above the threshold | Finds the missing personal financial statement |
| **Projection linkage** | Projected growth vs. use of proceeds | Finds projections that do not connect to the loan |
| **Date coherence** | Every dated item vs. the target submission date | Catches staleness before the lender does |
| **Collateral agreement** | Collateral schedule vs. appraisals vs. liens | Prevents a surprise lien position |
| **Entity agreement** | Borrowing entity vs. tax returns vs. formation documents | Catches a mismatched legal name |
| **Injection agreement** | Stated injection vs. evidenced injection | Catches an unevidenced or borrowed injection |

---

# 50. The Reconciliation Memo

Where a difference is real and explainable, the engine drafts a **reconciliation memo** for the owner to approve.

The memo states:

* what the two figures are
* which documents they appear in
* why they differ — timing, non-operating items, classification, or a correction
* whether the difference is permanent or a timing effect

> **Never leave the lender to discover a difference. A difference explained in advance is diligence. The same difference found by the underwriter is doubt.**

The engine drafts this. The owner approves it. The engine never asserts a reconciliation reason the owner has not confirmed.

---

# 51. Staleness Rules

Every dated item carries a staleness window, taken from the requirement profile.

```text
staleness = submission_date - item_as_of_date
```

Typical windows:

| Item | Typical window |
| --- | --- |
| Interim profit and loss statement | 90 days |
| Interim balance sheet | 90 days |
| Bank statements | Most recent months |
| Personal financial statement | Per profile |
| Appraisal | Per profile and collateral type |
| Environmental assessment | Per profile |
| Tax returns | Most recent completed year |

The engine reports the days remaining before each item goes stale, so the owner can see the submission window closing.

> **A packet is assembled against a target submission date. If that date slips, the engine re-evaluates every item rather than assuming the packet is still good.**

---

# 52. Signatures and Execution

The engine tracks, per form:

* rendered
* sent for signature
* signed
* notarized, where required
* witnessed, where required
* returned and verified

An unexecuted mandatory form is a hard block on submission.

The engine also checks the common silent failures:

* a signature page present but unsigned
* a signature present but undated
* an unsigned tax-return acknowledgment page
* a notary block incomplete

---

# 53. The Cover Memo

A single page, drafted by the engine from platform facts, approved by the owner.

It contains:

* the request, in one sentence
* the purpose
* the packet contents, by section
* the owner's advisors and their contact details
* any known gap and how it is being addressed
* any reconciliation the lender should expect

> **The cover memo is where the owner gets to frame the request before the lender forms a view. Leaving it out is a wasted opportunity, not a neutral omission.**

---

# 54. File Naming and Organization

The engine enforces a naming convention so the packet is navigable and so that a re-submission supersedes cleanly rather than colliding.

```text
<Section Number>_<Document Type>_<Period>_<Entity>.<ext>

03_Business Tax Return_2024_Acme LLC.pdf
06_Profit and Loss_YTD Mar 2026_Acme LLC.pdf
07_Debt Schedule_as of Mar 2026_Acme LLC.pdf
09_Personal Financial Statement_2026_Jane Doe.pdf
```

Rules:

* Section number first, so the folder sorts in packet order
* Document type named as the requirement names it, not as the owner filed it
* Period stated unambiguously
* Entity named, so multi-entity packets do not collide
* No spaces in the path where a portal will reject them; the engine follows the recipient's convention where one is specified

---

# 55. The Follow-Up Log

Submission is not the end. Lenders ask questions.

The engine maintains a follow-up log per submission:

* what was requested
* when
* by whom
* which item or requirement it maps to
* status
* the response and when it was sent

This is what turns a second request from a scramble into a lookup.

---

# 56. Packet Versioning and Reproduction

Every packet is versioned. Every version is reproducible.

A packet version records:

* the requirement profile and its version
* every item, with its content hash
* the derived schedules and their inputs
* the consistency check results
* the gate decision
* the reconciliation memos

> **A packet that cannot be reproduced cannot be defended.** When a lender asks what was submitted, the answer must be the actual document, not a reconstruction.

---

# 57. What the Owner Sees

A single readiness view:

* **Submittable** — nothing blocking
* **Almost** — specific gaps, each with the action that closes it and the party responsible
* **Not yet** — the blocking gaps, in the order that matters

Plus:

* the days remaining before each dated item goes stale
* the submission target date and whether it is still achievable
* what the lender has asked for since submission

> **The owner should never learn from the lender that something was missing.**

---

# 58. What the Lender Sees

A complete, ordered, indexed packet where:

* every required item is present
* every figure traces to a source
* every difference is explained
* every form is executed
* every dated item is current
* the cover memo frames the request
* the index resolves to pages

The lender's remaining work is judgment, not archaeology.

---

# 59. Core Data Objects

## UnderwritingPacket

The assembled submission artifact.

```text
UnderwritingPacket
  packet_id
  transaction_id
  business_id
  program                 (SBA_7A | SBA_504 | CONVENTIONAL | INVESTOR | CUSTOM)
  institution_reference   (optional)
  requirement_profile_id
  requirement_profile_version
  target_submission_date
  status                  (DRAFT | ASSEMBLING | NOT_SUBMITTABLE | SUBMITTABLE | SUBMITTED | SUPERSEDED)
  sections[]              (PacketSection)
  gate_result             (GateResult)
  consistency_results[]   (ConsistencyCheck)
  reconciliation_memos[]  (ReconciliationMemo)
  version
  created_at
  assembled_at
```

## LenderRequirementProfile

The declared requirements for a program and institution.

```text
LenderRequirementProfile
  profile_id
  program
  institution_reference   (optional)
  version
  effective_date
  superseded_date         (optional)
  source_reference        (citation for the requirements)
  requirements[]          (PacketRequirement)
  owner_engine            (which engine maintains this profile)
  reviewed_at
```

## PacketRequirement

One requirement in a profile.

```text
PacketRequirement
  requirement_id
  profile_id
  name
  description
  category                (PLATFORM_FACT | UPLOADED_DOCUMENT | EXECUTED_FORM |
                           THIRD_PARTY_REPORT | DERIVED_SCHEDULE | NARRATIVE | ATTESTATION)
  mandatory               (bool)
  condition               (expression, when conditional)
  staleness_window_days   (optional)
  requires_signature      (bool)
  requires_notarization   (bool)
  requires_certification  (bool)
  supplied_by             (engine key or party)
  section_number
  evidence_expected
```

## RequirementSource

The artifact that satisfies a requirement.

```text
RequirementSource
  source_id
  requirement_id
  packet_id
  source_type             (PLATFORM_FACT | DOCUMENT | FORM | REPORT | DERIVATION | NARRATIVE | ATTESTATION)
  source_engine           (optional)
  source_object_id        (optional)
  document_reference      (optional, Local Vault)
  as_of_date
  content_hash
  asserted_by             (party, where owner-provided)
  evidenced               (bool)
  satisfaction            (SATISFIED | GAP | NOT_APPLICABLE | BLOCKED)
  gap_reason              (specific, when not satisfied)
```

## PacketSection

```text
PacketSection
  section_id
  packet_id
  number
  title
  purpose
  required_by_profile     (bool)
  items[]                 (PacketItem)
  ordering_rule
  status
```

## PacketItem

```text
PacketItem
  item_id
  section_id
  item_number            (stable, referenced from the narrative)
  label
  source                  (RequirementSource)
  page_range              (optional)
  included                (bool)
  exclusion_reason        (where deliberately omitted)
```

## ConsistencyCheck

```text
ConsistencyCheck
  check_id
  packet_id
  check_type              (REVENUE_AGREEMENT | USE_SOURCE_BALANCE | ...)
  status                  (AGREE | DISAGREE | INSUFFICIENT_DATA | EXPLAINED)
  left_value
  left_source
  right_value
  right_source
  difference
  reconciliation_memo_id  (optional)
  severity
```

## ReconciliationMemo

```text
ReconciliationMemo
  memo_id
  packet_id
  consistency_check_id
  explanation
  cause                   (TIMING | NON_OPERATING | CLASSIFICATION | CORRECTION | OTHER)
  drafted_by              (engine)
  approved_by             (owner)
  approved_at
```

## UnderwritingSubmission

The record of what was sent.

```text
UnderwritingSubmission
  submission_id
  packet_id
  packet_version
  recipient               (institution or party)
  recipient_contact
  submitted_at
  submitted_by
  method
  item_manifest[]         (item_number, label, content_hash)
  disclosures[]           (consent records under which items were shared)
  acknowledgment_reference (optional)
```

## FollowUpRequest

```text
FollowUpRequest
  request_id
  submission_id
  requested_by
  requested_at
  description
  mapped_requirement_id   (optional)
  mapped_item_number      (optional)
  status                  (OPEN | ANSWERED | WAIVED)
  response_reference
  responded_at
```

## BusinessDebtSchedule

A derived roll-up owned by this engine.

```text
BusinessDebtSchedule
  schedule_id
  business_id
  as_of_date
  obligations[]           (creditor, type, original_amount, balance, rate,
                           monthly_payment, maturity, collateral, status,
                           in_coverage_calculation, exclusion_reason)
  total_monthly_debt_service
  prepared_by             (engine)
  handed_to               (Financial Modeling)
```

## CollateralSchedule

```text
CollateralSchedule
  schedule_id
  business_id
  as_of_date
  items[]                 (description, type, owner, valuation, valuation_source,
                           valuation_date, existing_liens, proposed_position,
                           insurance_reference)
  total_estimated_value
```

## EquityInjectionRecord

```text
EquityInjectionRecord
  record_id
  packet_id
  guarantor_id
  amount
  source_type             (SAVINGS | SALE_PROCEEDS | RETIREMENT | GIFT | OTHER)
  source_description
  evidence_reference
  seasoned_since
  is_borrowed             (bool, with evidence)
  asserted_by
  verified                (bool)
```

---

# 60. The Engine Contract

```text
assemble_packet(transaction_id, program, institution?, target_submission_date?)
    -> UnderwritingPacket

resolve_requirements(packet_id, profile_id)
    -> [RequirementSource]

check_consistency(packet_id)
    -> [ConsistencyCheck]

gate(packet_id)
    -> GateResult { status, gaps[], blocks[] }

render(packet_id, version)
    -> RenderedPacket

record_submission(packet_id, version, recipient, method)
    -> UnderwritingSubmission

log_follow_up(submission_id, description, mapped_item?)
    -> FollowUpRequest
```

Every operation is idempotent. `assemble_packet` on unchanged inputs and an unchanged profile version produces an identical packet.

---

# 61. Events Emitted

```text
UnderwritingPacketCreated
RequirementProfileResolved
RequirementSatisfied
RequirementGapIdentified
ConsistencyCheckFailed
ConsistencyCheckReconciled
PacketGateEvaluated
PacketMarkedSubmittable
PacketMarkedNotSubmittable
PacketRendered
PacketSubmitted
FollowUpRequestReceived
FollowUpRequestAnswered
PacketSuperseded
PacketStaleItemDetected
```

---

# 62. Relationship to the Capital / Financing Engine

**Capital** decides which financing structures and lenders to pursue, and coordinates the capital stack.

**Underwriting** produces the packet a selected lender requires.

```text
CAPITAL / FINANCING
        │  selects: program, institution, structure
        ▼
UNDERWRITING ENGINE
        │  resolves requirements, assembles, gates
        ▼
PACKET → LENDER
```

Capital says *where to apply*. Underwriting says *what to send*.

Underwriting never chooses the lender. If no program has been selected, it cannot assemble a packet, and it says so rather than guessing.

---

# 63. Relationship to the Review Package Engine

This is the closest boundary in the platform, and it must be stated precisely.

| | Review Package Engine | Underwriting Engine |
| --- | --- | --- |
| **Owns** | The generic package model — purpose, sections, relevance, stakeholder-specific assembly | Conformance to an external lender's underwriting standard |
| **Standard** | The platform's own judgment about what a professional needs | The lender's declared requirements |
| **Gate** | Is the package useful to this reader? | Is the packet submittable to this lender? |
| **Audience** | Any stakeholder — attorney, CPA, advisor, lender | A specific lender program or institution |
| **Output** | A review package | A submission packet |

A review package **for a lender** is a preliminary document that helps an owner discuss financing. An underwriting packet is a **submission** that a lender can act on.

```text
REVIEW PACKAGE ENGINE
   builds the generic package, any audience
        │
        ▼
UNDERWRITING ENGINE
   applies the lender's standard, gates, and records submission
```

Underwriting does not re-implement package assembly. It applies conformance on top of it, and where a section is generic it draws on the package model rather than duplicating it.

---

# 64. Relationship to Document Readiness

**Document Readiness** owns whether a document exists, is complete, and is ready, from the platform's perspective.

**Underwriting** owns whether a document satisfies a *lender's* requirement, including requirements the platform's own readiness model does not track.

```text
DOCUMENT READINESS          UNDERWRITING
  is the document ready?      does it satisfy requirement R,
                              for program P, as of date D?
```

A document can be ready and still fail a lender requirement — because the lender requires three years and the platform has two, or because the lender requires notarization and the document is merely signed.

Underwriting consumes readiness state; it does not recompute it.

---

# 65. Relationship to the Financial Modeling Engine

Financial Modeling owns every financial calculation. Underwriting owns none.

```text
UNDERWRITING  ──requests: DSCR, global DSCR, affordability──▶  FINANCIAL MODELING
UNDERWRITING  ◀──returns: referenced figures with basis──     FINANCIAL MODELING
```

Underwriting supplies the business debt schedule, which Financial Modeling needs as an input to the coverage calculation. It receives back a referenced result — never a bare number — so that every figure in the packet can be traced to the assumption and version that produced it.

Underwriting never adjusts a projection to make coverage work.

> **If the packet only balances because a number was softened, the packet is a misrepresentation.**

---

# 66. Relationship to the Valuation Engine

Underwriting presents the valuation and the price. It does not produce or opine on either.

Where a program requires a certified or independent valuation, Underwriting records that requirement and tracks the report as a third-party item. Where the transaction relies on a platform estimate, Underwriting presents it as an estimate, with its range and method summary — never as a determination.

> **A lender must never be handed a platform estimate that reads as a certified appraisal.**

---

# 67. Relationship to the Evidence Ledger and Fact Verification

* **Evidence Ledger** supplies the source and authority behind each fact, so a presented figure can carry its provenance into the packet.
* **Fact Verification** owns conflicts between sources. Underwriting **reports** a conflict it finds in the packet; Fact Verification **owns** the record of the conflict and its resolution.

Underwriting's consistency checks are packet-scoped. Fact Verification's are source-scoped. When they disagree, Fact Verification's record is authoritative.

---

# 68. Relationship to Consent & Access and the Local Vault

* **Local Vault** holds the documents and their versions. Underwriting references them and never copies them.
* **Consent & Access** decides who may receive which document, for what purpose, until when.

Before any item leaves the platform, the engine checks disclosure permission. A requirement that is satisfied but not permitted to be shared is reported as a **blocked** requirement, naming the consent that is missing.

> **The engine will not send a document the owner has not authorized sending — even to make the packet complete.**

---

# 69. Worked Example — SBA 7(a) Acquisition

An owner acquires a competitor for $2,400,000, with $2,000,000 requested.

```text
PROGRAM SELECTION
  program = SBA_7A
  profile = sba_7a_v<current>, effective <date>, sourced <citation>

REQUIREMENT RESOLUTION
  1919, 912 (2 owners ≥20%), 413 (2), Form 4 (lender)
  3 years business tax returns
  3 years personal tax returns (2 owners)
  interim P&L and balance sheet within 90 days
  2+ months bank statements
  business debt schedule
  2-year projections
  business plan, resumes
  formation docs, operating agreement, licenses, lease, EIN
  IDs, acquisition package, seller 3-year financials, AR/AP aging,
  certified valuation, environmental (collateral-triggered)

SOURCE RESOLUTION
  satisfied   : 34 requirements
  gap         : personal 2023 return, owner B  ← the classic single-owner gap
  blocked     : equipment appraisal — consent not yet granted to the lender

DERIVATION
  debt schedule, source and use, collateral schedule, ownership chart

CONSISTENCY CHECKING
  DISAGREE  revenue: 2024 tax return $4.12M vs P&L $4.31M
            → reconciliation memo drafted: timing of a December invoice
  DISAGREE  debt: bank statement shows a lease payment absent from the schedule
            → real gap, schedule corrected, coverage recalculated
  AGREE     use = source, $2,400,000

GATING
  status = NOT_SUBMITTABLE
  blocks: personal 2023 return (owner B)
          equipment appraisal consent
  note  : every block names a party and an action

AFTER RESOLUTION
  status = SUBMITTABLE
  rendered, indexed, submitted, recorded
```

The owner learns two things that would otherwise have cost weeks: one tax return is missing, and one appraisal cannot be shared without consent.

---

# 70. Worked Example — Conventional Bank Expansion

A long-standing customer requests $600,000 for equipment and working capital.

The conventional profile is thinner on forms and heavier on the bank's own credit policy.

```text
REQUIREMENT RESOLUTION
  program = CONVENTIONAL, institution = <named bank>
  profile drawn from the bank's declared checklist, not a program SOP

  differences from the SBA profile:
    no government forms
    2 years of tax returns, not 3
    4 months of bank statements, not 2
    bank's own financial statement template required
    collateral coverage emphasized; appraisal required for the equipment

CONSISTENCY CHECKING
  AGREE on revenue, debt, ownership
  INSUFFICIENT_DATA on projected coverage — the bank requires a
    covenant-style projection the platform does not yet hold

GATING
  status = NOT_SUBMITTABLE
  block: equipment appraisal outstanding
  gap  : covenant projection, owner to supply the bank's template
```

The engine surfaces the bank-specific template requirement early, rather than after the owner has already assembled everything to the SBA's shape.

---

# 71. Worked Example — Investor-Backed Buyout

An investor group is acquiring a services business. The packet is a due diligence file, not a loan file.

```text
PROGRAM SELECTION
  program = INVESTOR
  profile = eight workstreams, 171-item scope

REQUIREMENT RESOLUTION
  financial (25) legal (26) operational (20) HR (25)
  technology (25) commercial (20) environmental (15) regulatory (15)

  critical items surfaced first:
    quality of earnings
    AR and AP aging
    debt schedule with covenants
    EBITDA adjustments and add-backs
    cap table
    material contracts
    customer concentration
    key person risk
    IP assignment agreements
    SOC 2 or security audit
    Phase I environmental site assessment
    customer reference calls

CONSISTENCY CHECKING
  DISAGREE  EBITDA: management-adjusted EBITDA vs QofE-adjusted EBITDA
            → material, and the single most important number in the deal
  DISAGREE  customer concentration: top customer at 31% of revenue,
            not disclosed in the management summary
            → factual gap, escalated rather than reconciled

GATING
  status = NOT_SUBMITTABLE
  blocks: QofE not yet commissioned (third-party, external lead time)
          Phase I ESA not ordered
  note  : the engine reports lead times, because these cannot be
          closed by the owner alone
```

The engine's value here is that the two most consequential findings — an EBITDA disagreement and an undisclosed concentration — surface **before** the investor finds them.

---

# 72. Anti-Patterns

## 72.1 The Shape-Shifting Packet

Rendering a different packet each time, so no one can say what was submitted.

**Correct:** version and reproduce every packet exactly.

## 72.2 The Silent Gap

Reporting "financials incomplete" instead of naming the missing year and owner.

**Correct:** name every gap specifically, with the party who closes it.

## 72.3 The Invented Requirement

Adding a requirement because it seems sensible, without a source.

**Correct:** requirements come from a sourced, versioned profile or they do not exist.

## 72.4 The Unsourced Figure

Presenting a number in the packet without saying where it came from.

**Correct:** every presented figure carries its source and as-of date.

## 72.5 The Comfortable Projection

Softening a projection so coverage clears.

**Correct:** present the projection as built and let the coverage fall where it falls.

## 72.6 The Hidden Reconciliation

Leaving the lender to find a revenue difference.

**Correct:** explain it in advance, in a memo the owner approved.

## 72.7 The Estimate Dressed as an Appraisal

Presenting a platform valuation range where the program requires a certified determination.

**Correct:** label every figure with what it actually is.

## 72.8 The Consent Bypass

Completing the packet by sending a document the owner never authorized sharing.

**Correct:** a requirement that is satisfied but not permitted is blocked, and says so.

## 72.9 The Stale Submission

Submitting a packet whose interim financials aged out while the target date slipped.

**Correct:** re-evaluate staleness against the actual submission date, every time.

## 72.10 The Single-Program Assumption

Assuming the deal is an SBA deal because it is a business acquisition.

**Correct:** require explicit program selection; suggest, but do not infer.

## 72.11 The Forgotten Guarantor

Missing a personal financial statement for a 20%-plus owner who is not operationally involved.

**Correct:** derive the guarantor set from ownership records, not from who is in the conversation.

## 72.12 The Appendix Dump

Assembling every document the platform holds, ordered by arrival.

**Correct:** include what the profile requires, ordered by the packet structure, with deliberate exclusions recorded.

---

# 73. What the Engine Should Never Do

It should never:

* Invent, guess, or infer a lender's requirement
* Gate a packet against a requirement profile with no source
* Compute a financial figure owned by Financial Modeling
* Present a platform estimate as a certified determination
* Adjust a projection, a value, or a schedule to improve an outcome
* Assert a reconciliation reason the owner has not approved
* Send a document that Consent & Access has not permitted
* Hide a gap, a block, or a conflict
* Mark a packet submittable while a mandatory requirement is unsatisfied
* Submit an unsigned or unexecuted mandatory form
* Submit a packet whose uses and sources do not balance
* Submit interim financials outside their staleness window
* Treat an owner's assertion as an evidenced fact without saying so
* Present platform-generated narrative as the owner's own words
* Decide creditworthiness, eligibility, or approval
* Replace the lender's own underwriting

---

# 74. Observability

The engine reports:

* packet status, per packet and per section
* requirement satisfaction rate, by category and by program
* the specific gap list, always with the responsible party
* consistency check results, by check type
* staleness countdown for every dated item
* gate outcomes, and the reasons for each block
* time from assembly start to submittable
* follow-up requests per submission, and time to answer
* the most frequently missing requirements, across transactions

That last metric is a product signal, not just an operational one: if the same requirement is missing in most transactions, the platform should be collecting it earlier.

---

# 75. Performance Considerations

* Assembly is incremental — a re-run after a single document change does not re-derive the whole packet
* Consistency checks are scoped to the items that changed and the checks that depend on them
* Rendering is cached per packet version; an unchanged version is not re-rendered
* Large document sets are indexed rather than concatenated, so the packet stays navigable
* Requirement resolution is cached per profile version, and invalidated when a profile is superseded

---

# 76. Extensibility

New programs and institutions are added as **requirement profiles**, never as code.

Adding a new lender profile requires:

* a program or institution identifier
* a version and effective date
* a source reference
* the requirement list, with conditions and staleness windows
* section mappings

Adding a new consistency check requires:

* the two things being compared
* the tolerance
* the severity
* whether a reconciliation memo can resolve it

No engine change is required to onboard a lender.

---

# 77. Architectural Lock

1. The engine owns conformance to an external standard, not the substance being conformed.
2. Requirements are sourced, versioned data. No requirement exists without a citation.
3. A packet is never gated against an unsourced profile.
4. Financial figures are requested from their owning engine, never computed here.
5. Every presented figure carries its source and as-of date.
6. Asserted facts are labelled as asserted until evidenced.
7. Every gap is named specifically, with the party who closes it.
8. Every block is named specifically, with the action that clears it.
9. Completeness gating is a hard gate. There is no silent override.
10. A recorded override names the authorizer, the reason, and what the lender was told.
11. Uses must equal sources before submission.
12. Interim financials outside their staleness window block submission.
13. Mandatory unsigned or unexecuted forms block submission.
14. A satisfied but unpermitted requirement is blocked, not satisfied.
15. No document leaves the platform without a disclosure permission.
16. Consistency differences are either reconciled and approved or they block.
17. A reconciliation memo is drafted by the engine and approved by the owner.
18. Packets are versioned and exactly reproducible.
19. Every submission is recorded with its item manifest and content hashes.
20. Follow-up requests are logged and mapped to the requirement they concern.
21. The engine never adjusts a number to improve an outcome.
22. The engine never presents an estimate as a determination.
23. The engine never decides creditworthiness.
24. The engine never replaces the lender's underwriting.
25. The engine never infers the program; program selection is explicit.

---

# 78. Architectural Boundary Summary

| Engine | Owns | Does Not Own |
| --- | --- | --- |
| **Business Reality** | Current business facts and financial statements | Lender requirements and packet assembly |
| **Financial Modeling** | Debt service coverage, global coverage, affordability | The packet that presents the result |
| **Valuation** | Value ranges and professional determinations | The lender's collateral or credit decision |
| **Capital / Financing** | Financing structures and lender selection | The submission packet |
| **Document Readiness** | Whether a document exists, is complete, and is ready | Whether it satisfies a lender's requirement |
| **Review Package** | Generic stakeholder package assembly | Program-specific conformance and submission |
| **Evidence Ledger** | Provenance and authority of each fact | Packet ordering and gating |
| **Fact Verification** | Conflicts between sources | Presenting reconciled figures to a lender |
| **Professional Review** | Professional determinations | Underwriting standards |
| **Consent & Access** | Who may receive which document, and for how long | What the lender requires |
| **Local Vault** | Document storage, encryption, and versioning | Packet content and structure |
| **Identity & Access** | Who a party is and what they may do on the platform | Lender requirement conformance |
| **Notification** | Delivery of attention, including follow-up alerts | The follow-up record itself |
| **Audit / Provenance** | The historical record of what happened | The packet's contents |
| **Transaction / Orchestration** | Execution of the transaction | Preparation of the financing submission |
| **Underwriting** | Lender requirement profiles, packet assembly, conformance gating, submission records | Business facts, financial calculations, valuations, consent, the credit decision |

---

# 79. The Engine's Place in the Platform

```text
BUSINESS REALITY ──▶ facts
        │
VALUATION ──▶ value ──┐
        │             │
SCENARIO ──▶ paths ───┤
        │             │
FINANCIAL MODELING ──▶ coverage, affordability
        │
CAPITAL / FINANCING ──▶ program, institution, structure
        │
        ▼
UNDERWRITING ENGINE
        │
        ├── resolves the lender's requirements
        ├── sources every item from its owner
        ├── derives the debt, use-and-source, collateral, and ownership schedules
        ├── checks the packet against itself
        ├── gates on completeness, currency, and consistency
        ├── renders an ordered, indexed packet
        └── records exactly what was submitted
        │
        ▼
PACKET → LENDER
        │
        ▼
CREDIT DECISION  (the lender's, not the platform's)
        │
        ▼
CLOSING ENGINE
```

The engine sits at the boundary between the platform's knowledge and an outside party's judgment. Everything before it is the platform deciding what is true. Everything after it is someone else deciding what to do about it.

Its single obligation is to make that handoff complete, current, consistent, and traceable — so the answer turns on the business rather than on the paperwork.

> **The engine estimates nothing, decides nothing, and promises nothing. It assembles the case so completely that the only thing left to discuss is whether the business deserves the loan.**
