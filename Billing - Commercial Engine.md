# Billing / Commercial Engine

## 1. Purpose

The Billing / Commercial Engine manages how the platform makes money, charges customers, recognizes subscriptions and commercial entitlements, and administers commercial relationships.

It handles:

* Plans and pricing
* Subscriptions
* One-time charges
* Usage-based charges
* Transaction-related fees where permitted
* Invoices
* Payments
* Refunds and credits
* Coupons and promotions
* Entitlements
* Commercial accounts
* Billing contacts
* Tax-related billing data supplied by payment providers
* Commercial reporting
* Billing events
* Subscription lifecycle
* Revenue-related platform metadata
* Commercial policy controls

Its central question is:

> **“What commercial relationship does this customer have with the platform, what are they entitled to, what do they owe, and what has been paid?”**

The engine manages the platform's commercial relationship. It does not control the substantive transaction.

---

# 2. Core Architectural Principle

## Monetization must remain separate from transaction judgment.

The commercial engine must never allow money flows or commercial incentives to silently influence:

* Professional matching
* Vendor ranking
* Scenario selection
* Professional determinations
* Financing outcomes
* Confidence calculations
* Research conclusions
* Transaction recommendations

A provider should not receive preferred placement because they pay more.

A customer should not receive a different substantive recommendation because they are on a more expensive plan unless the difference is explicitly a product entitlement and clearly disclosed.

Commercial logic governs access to commercial features, not truth.

---

# 3. What This Engine Owns

* Product catalog
* Plans
* Price versions
* Subscription records
* Commercial accounts
* Billing profiles
* Payment references
* Invoices
* Credits
* Refunds
* Commercial entitlements
* Usage records used for billing
* Commercial discounts
* Commercial lifecycle
* Billing events
* Revenue reporting metadata
* Commercial agreements/reference records
* Commercial exceptions

---

# 4. What It Does Not Own

It does not own:

* Identity
* Authentication
* Sensitive-data access
* Professional fees charged independently by professionals
* Vendor vetting
* Transaction state
* Financial modeling
* Seller-note economics
* Legal agreements
* Accounting books of record

External payment providers may remain the payment system of record for actual payment processing. The Billing Engine maintains the platform's commercial representation of those transactions.

---

# 5. Commercial Account

A Commercial Account represents the customer relationship with the platform.

Possible customers:

* Individual owner
* Business
* Professional organization
* Enterprise customer
* Platform partner
* Other authorized organization

A commercial account contains:

* Account ID
* Customer identity reference
* Billing entity
* Billing contact
* Plan
* Status
* Entitlements
* Payment-provider references
* Created date
* Renewal date
* Cancellation state

---

# 6. Product Catalog

The engine should maintain a formal product catalog.

Examples:

* Core platform subscription
* Professional workspace
* Advanced research
* Premium workflow features
* Additional seats
* Storage tiers
* Enterprise administration
* Optional professional tools

Products should be versioned.

A product should not be silently redefined while existing customers are using it.

---

# 7. Pricing

Pricing should be represented separately from products.

A product can have:

* Monthly price
* Annual price
* Seat-based price
* Usage-based price
* One-time price
* Introductory price
* Enterprise/custom price

Every price has an effective date and version.

---

# 8. Commercial Plans

Plans define bundled entitlements.

Example:

**Starter**

* Core Journey
* Local Vault
* Basic workflow
* Limited professional packages

**Professional**

* Advanced collaboration
* Expanded workflow
* Enhanced research

**Enterprise**

* Organization administration
* Advanced governance
* Custom integrations

These are illustrative structures. The actual commercial catalog should be designed separately from core engine architecture.

---

# 9. Entitlements

An Entitlement answers:

> What is this customer allowed to use because of their commercial relationship?

Examples:

* Number of active workspaces
* Number of seats
* Research usage allowance
* Storage allowance
* Advanced reporting
* API access
* Integration access

Entitlements are not permissions.

Identity & Access determines whether a user may perform an action.

Billing determines whether the customer's commercial plan includes the feature.

Policy determines whether the action is permitted.

---

# 10. Subscription Lifecycle

Suggested states:

* Trial
* Active
* Past Due
* Paused
* Cancelled
* Expired
* Expired Grace Period
* Terminated

The platform should preserve history when subscription state changes.

---

# 11. Grace Periods

A payment failure should not necessarily immediately destroy the user's access.

The engine can support:

> Payment failed → grace period → warning → restricted commercial features → cancellation.

Policy determines what can continue operating during the grace period.

Local Vault data must not be held hostage to a billing outage.

A user should not lose access to locally stored private records simply because a subscription payment failed.

---

# 12. Local-First Commercial Principle

Billing status should affect commercial entitlements, not ownership of user data.

For example:

Subscription expires.

The platform may disable premium cloud functionality.

It should not imply:

> “We now own your business documents.”

The Local Vault remains governed by its own data and lifecycle policies.

---

# 13. Invoices

Invoices should contain:

* Invoice ID
* Customer
* Billing period
* Line items
* Quantity
* Unit price
* Discounts
* Taxes as supplied/calculated by the payment provider or configured tax system
* Total
* Currency
* Status
* Payment reference
* Issue date
* Due date

The engine should distinguish invoice status from payment status.

---

# 14. Payment Events

Examples:

* Payment initiated
* Payment authorized
* Payment succeeded
* Payment failed
* Payment refunded
* Charge disputed

The payment provider remains authoritative for the payment event where applicable.

Billing maintains synchronized commercial state.

---

# 15. Refunds and Credits

The engine should support:

* Full refund
* Partial refund
* Account credit
* Promotional credit
* Commercial adjustment

Every adjustment should have:

* Reason
* Actor
* Amount
* Related invoice
* Timestamp
* Approval/reference where required

---

# 16. Usage-Based Billing

Some future platform features may be usage-based.

Examples:

* Research runs
* AI processing units
* Additional storage
* API calls
* Premium exports

Usage should be recorded as metered events.

The Billing Engine converts those events into commercial charges.

It should not become the usage engine itself.

---

# 17. Commercial Events

Examples:

```text
SubscriptionStarted
SubscriptionRenewed
SubscriptionCancelled
PaymentSucceeded
PaymentFailed
TrialEnding
InvoiceCreated
InvoicePaid
RefundIssued
EntitlementGranted
EntitlementExpired
CommercialAccountSuspended
```

Workflow and Notification can consume these events.

---

# 18. Transaction-Related Fees

The platform may eventually charge transaction-related fees.

This deserves special governance because transaction-related compensation can create conflicts of interest.

The Billing Engine can represent:

* Fixed platform fee
* Subscription fee
* Transaction service fee
* Professional software fee
* Other commercial charge

But it must not silently convert provider referral economics into ranking or recommendation logic.

Any referral or transaction-based compensation model should be subject to explicit Policy / Compliance review before being enabled.

---

# 19. Marketplace Separation

A particularly important architectural rule:

Vendor commercial status must not automatically influence marketplace ranking, matching, or publication.

For example:

> Vendor pays for premium software tools.

That does not mean:

> Vendor appears first in professional matching.

Marketplace remains governed by its own curation and matching rules.

---

# 20. Commercial Exceptions

Authorized administrators may need to create:

* Discounts
* Credits
* Custom pricing
* Extended trials
* Payment arrangements

Exceptions should be explicit, time-bound where appropriate, and auditable.

---

# 21. Commercial Reporting

The engine can provide:

* MRR/ARR metadata
* Subscription counts
* Churn
* Trial conversion
* Payment success
* Refunds
* Credits
* Revenue by product
* Commercial account status

Accounting remains the financial system of record for the company's books.

Billing is the platform's commercial system of record, not the corporate general ledger.

---

# 22. Entitlement Changes

When a subscription changes:

```text
SubscriptionChanged
      ↓
Billing
      ↓
EntitlementUpdated
      ↓
Identity / Access
      ↓
Feature availability changes
```

This must never remove unrelated security permissions or user identity information.

---

# 23. Cancellation

Cancellation should preserve:

* Current plan
* Effective termination date
* Remaining entitlements
* Commercial history
* Invoice history
* Payment history

The user experience should clearly distinguish:

> Cancel at period end

from:

> Cancel immediately.

---

# 24. Dunning

A payment-recovery workflow can use:

* Reminder
* Retry
* Grace period
* Escalation
* Suspension
* Cancellation

Workflow handles the execution mechanics.

Notification handles delivery.

Billing supplies the commercial state.

---

# 25. Security

Billing handles sensitive commercial data such as:

* Billing identifiers
* Payment-provider references
* Invoice information
* Commercial contracts

It should minimize storage of raw payment credentials where possible and rely on payment providers/tokenization for card/payment details.

---

# 26. Core Data Objects

## CommercialAccount

Customer's commercial relationship.

## Product

Platform offering.

## Price

Versioned price definition.

## Plan

Bundle of products/entitlements.

## Subscription

Customer's active commercial agreement with the platform.

## Entitlement

Feature/usage right granted commercially.

## Invoice

Commercial charge record.

## PaymentReference

Reference to external payment-provider state.

## Credit

Commercial credit or adjustment.

## Refund

Refund record.

## UsageRecord

Metering input for usage billing.

## CommercialException

Approved deviation from standard commercial terms.

---

# 27. Engine Contract

```text
createCommercialAccount()
createProduct()
createPrice()
createPlan()
createSubscription()
changeSubscription()
cancelSubscription()
pauseSubscription()
updateEntitlements()
createInvoice()
recordPayment()
recordRefund()
issueCredit()
recordUsage()
getCommercialStatus()
getEntitlements()
getInvoiceHistory()
getPaymentHistory()
```

---

# 28. Architectural Lock

* Billing / Commercial is a standalone engine.
* It manages platform monetization, not transaction substance.
* Products and prices are versioned.
* Subscriptions are versioned and lifecycle-aware.
* Commercial entitlements are distinct from identity permissions.
* Payment providers may remain authoritative for actual payment processing.
* Billing maintains the platform's commercial representation of payments.
* Payment failures do not imply ownership of local user data.
* Vendor payments do not influence marketplace ranking or professional matching.
* Transaction-related compensation requires explicit governance.
* Commercial exceptions are explicit and auditable.
* Billing activity is available to Audit.
* Billing events are available to Workflow and Notification.
* The engine is independently versioned, tested, and replaceable.

---

# 29. Architectural Boundary Summary

| Engine                  | Owns                                                                     | Does Not Own                                                          |
| ----------------------- | ------------------------------------------------------------------------ | --------------------------------------------------------------------- |
| **Billing / Commercial** | Products, prices, subscriptions, invoices, payments, entitlements        | User identity, sensitive document access, transaction decisions       |
| **Identity & Access**   | Users, roles, platform permissions                                        | Commercial entitlement amounts                                        |
| **Policy / Compliance** | Commercial rules and restrictions                                         | Billing execution                                                     |
| **Marketplace**         | Vendor matching and visibility                                            | Vendor payment relationship                                           |
| **Vendor Vetting**      | Provider verification                                                     | Commercial billing                                                    |
| **Local Vault**         | User's private data                                                       | Commercial account status                                             |
| **Audit / Provenance**  | Billing history and changes                                               | Current subscription logic                                            |
| **Workflow**            | Billing reminders and commercial workflows                                | Commercial source of truth                                            |
| **Notification**        | Billing alerts                                                            | Payment state                                                         |

---

# 30. One-Sentence Definition

> **The Billing / Commercial Engine manages how customers pay for the platform and what commercially purchased capabilities they are entitled to use, without allowing monetization to influence substantive platform decisions.**
