Locked. 🔒
These are now **core journey-builder rules** for the app.
The journey engine should decide **how to ask**, not just **what to ask**:
> **Fact → Preference → Priority → Scenario → Professional Review → Professional Determination**
And for each question, the builder should select the appropriate interaction:
**Single select · Yes/No · Multi-select · Select up to 3 · Ranking · Trade-off · Numeric · Range · Percentage allocation · Choice + amount · Date/timeframe · Conditional choice · Scenario comparison · What-if · Confidence/flexibility · Not sure/help me decide · Optional explanation · Document selection · Review/confirmation**
The UX rules are equally locked:
> **2–3 choices at a time → progressive disclosure → explain why → reversible decisions → recalculate when changed → never force false certainty.**
The system should also preserve the distinction between:
**Owner Objective**
What the owner wants.
**Platform Scenario**
What might potentially accomplish it under stated assumptions.
**Professional Determination**
What the qualified professional concludes should actually be done.
And the journey builder should **avoid** giant forms, giant dropdowns, unnecessary open-ended questions, excessive ranking, and false precision.
### This gives us something important
We can now design the application around a **Journey Definition Engine** rather than hard-coding individual questionnaires.
A journey designer could essentially specify:
```text
Question
↓
Information Type
↓
Choose Interaction Pattern
↓
2–3 Choices
↓
Conditional Branch
↓
Next Question
↓
Update Owner Objective
↓
Generate / Modify Scenario
↓
Professional Review
```
That means later we could create a new journey, such as **Management Buyout**, without reinventing the entire application. The engine knows *how* to guide someone even when the subject matter changes.
I think this is the point where we have enough product rules to start defining the **actual journey-builder architecture and its first Employee Ownership journey**, rather than adding more conceptual features.
