This one is important because transactions are not just a collection of tasks and documents. They are also a **conversation between people who are trying to make a complicated thing happen**.

The engine should provide one coherent communication layer while preserving a strict distinction between:

> **Conversation**
> **Task**
> **Request**
> **Professional determination**
> **Notification**
> **Document**

Those things may be connected, but they should never become one giant blob.

# Communication Engine

## 1. Purpose

The Communication Engine manages secure, contextual communication among the people and organizations participating in a business ownership transition.

It handles:

* Secure messaging
* Questions
* Requests
* Stakeholder communications
* Transaction discussions
* Comments
* Message history
* Conversation context
* Attachments and references
* Mentions
* Responses
* Read status
* Conversation state

Its purpose is to give every transaction a **reliable communication record** without forcing email, text messages, chat threads, professional review notes, and task discussions into one undifferentiated system.

The central question is:

> **“Who is communicating with whom, about what, in what context, and what happened as a result?”**

---

# 2. Core Architectural Principle

## Communication records conversation. It does not own the work being discussed.

For example:

A lender says:

> “Please provide the updated debt schedule.”

Communication records the message.

Document Readiness records:

> Debt schedule requested.

Workflow may create:

> Owner task: Provide updated debt schedule.

Local Vault stores:

> DebtSchedule_v4.xlsx.

Consent & Access determines:

> Whether the lender may receive it.

Transaction Orchestration determines:

> Whether the missing debt schedule is blocking financing.

Each engine keeps its responsibility.

---

# 3. What This Engine Owns

The Communication Engine owns:

* Conversations
* Messages
* Threads
* Questions
* Requests
* Comments
* Replies
* Participants
* Conversation membership
* Message references
* Attachments/references
* Read state
* Message status
* Conversation status
* Mentions
* Message relationships
* Communication history

---

# 4. What It Does Not Own

It does not own:

* Document storage
* Document permissions
* Task management
* Transaction stages
* Professional determinations
* Business facts
* Notifications
* Marketplace profiles
* Financial calculations
* Decisions
* Audit history as a platform-wide source of truth

The Communication Engine may **reference** all of those things.

It should not become their system of record.

---

# 5. Communication vs Notification

This distinction should be locked.

### Communication

A participant is intentionally communicating with another participant.

> “I reviewed the valuation and have two questions.”

### Notification

The platform alerts someone that something happened.

> “Valuation review completed.”

Notification may link into Communication:

> “Valuation review completed. [Open discussion]”

But a notification is not a conversation.

The Communication Engine owns the discussion.

The Notification Engine owns delivery and alerting.

---

# 6. Communication vs Task

Another critical distinction:

### Communication

> “Can you send us the updated customer concentration report?”

### Task

> Owner: Provide customer concentration report. Due: October 14.

A message can **create or update a task**, but the task remains owned by Workflow / Transaction Orchestration.

This lets communication generate action without absorbing the action-management system.

---

# 7. Communication vs Professional Determination

This is especially important for the professional architecture.

A professional may write:

> “Based on our review, we recommend changing the proposed ownership structure.”

That is communication.

The formal professional conclusion belongs in:

> **Professional Review Engine → Professional Determination**

The communication can link to that determination.

This prevents an informal message from becoming accidental platform truth.

---

# 8. Communication vs Decision

Similarly:

> “I think we should delay closing.”

is a communication.

The owner's formal decision:

> “I choose to move the target closing date from November 30 to December 15.”

belongs in:

> **Decision Record Engine**

Communication should be able to point to the decision record.

---

# 9. Conversation Model

The fundamental object should be the **Conversation**.

A conversation contains:

* Conversation ID
* Workspace
* Transaction
* Subject
* Type
* Participants
* Visibility
* Status
* Related objects
* Created date
* Last activity
* Retention policy
* Sensitivity
* Current unread state

A conversation may be:

* Transaction-wide
* Stage-specific
* Professional-specific
* Participant-to-participant
* Task-specific
* Document-specific
* Question-specific
* Request-specific

---

# 10. Conversation Types

Examples:

### Transaction Conversation

> General transaction coordination.

### Professional Conversation

> Owner ↔ Attorney.

### Financing Conversation

> Owner + Lender + relevant participants.

### Valuation Conversation

> Owner + Valuation Professional.

### Document Conversation

> Discussion about a specific document.

### Task Conversation

> Discussion about completion of a particular task.

### Question Thread

> A discrete question requiring an answer.

### Request Thread

> A specific information/action request.

### Private Professional Conversation

> Restricted discussion between authorized parties.

---

# 11. Conversation Visibility

Conversations should support explicit visibility.

### Transaction-wide

Visible to authorized transaction participants.

### Team

Visible to a defined participant group.

### Private

Only explicitly invited participants.

### Professional-private

Restricted to designated professionals and authorized owner participants.

### Internal platform

Platform operational discussions, not visible to transaction participants.

Visibility is an access-policy concern and should integrate with **Consent & Access**.

The Communication Engine should not independently invent security rules.

---

# 12. Participants

A participant can be:

* Owner
* Co-owner
* Employee representative
* Management
* Attorney
* CPA
* Valuation professional
* Trustee
* Lender
* Financial advisor
* Buyer
* Seller-note purchaser
* Platform coordinator
* Other authorized stakeholder

Participants should be references to the **Stakeholder / Relationship Engine** rather than duplicate identity records.

---

# 13. Message

A Message contains:

* Message ID
* Conversation ID
* Sender
* Timestamp
* Body
* Message type
* Related objects
* Attachments/references
* Reply-to
* Mentions
* Sensitivity
* Status
* Version/edited state

Message types can include:

* Message
* Question
* Request
* Response
* Comment
* System update
* Professional note
* Decision reference

---

# 14. Questions

Questions should be first-class objects.

Example:

> **Question #Q-104**
>
> Does the valuation include the real estate subsidiary?

The system tracks:

* Asked by
* Assigned/responding participant
* Date asked
* Due date
* Status
* Related professional
* Related document
* Related fact
* Related scenario
* Response
* Resolution

States:

**Open**

**Assigned**

**Waiting**

**Answered**

**Accepted**

**Closed**

**Reopened**

This is much more useful than burying important questions in chat history.

---

# 15. Requests

Requests are also first-class.

Example:

> **Request #R-221**
>
> Please provide the 2025 debt schedule.

Requests contain:

* Requestor
* Recipient
* Purpose
* Resource/action requested
* Due date
* Related stage
* Related task
* Status
* Response
* Completion evidence

A request may originate in communication and trigger:

> Document Readiness + Workflow + Notification.

The Communication Engine records the conversation around it.

---

# 16. Comments

Comments are lightweight contextual communications.

Examples:

> “This number looks different from the prior version.”

> “I reviewed page 14.”

> “Please use the revised schedule.”

Comments can attach to:

* Document
* Fact
* Scenario
* Task
* Milestone
* Professional package
* Research finding
* Transaction

Comments should inherit the visibility rules of their parent object unless explicitly configured otherwise.

---

# 17. Threading

Messages should support replies.

Example:

```text
Owner:
"Do we need the updated lease before valuation?"

Attorney:
"Yes, because the lease affects the operating obligations."

Owner:
"Understood. I uploaded the current version."

Attorney:
"Received. I'll review it."
```

That becomes a coherent thread rather than four disconnected messages.

---

# 18. Conversation Linking

A conversation should be able to link to platform objects.

For example:

> Conversation #88
> Related transaction: TX-004
> Related valuation: VAL-0024
> Related task: TASK-812
> Related document: DOC-184

This produces contextual communication without copying the underlying objects.

---

# 19. Context Panel

When viewing a conversation, the platform should show related context.

Example:

**Discussion:** Valuation

**Related transaction:** TX-004

**Stage:** Valuation

**Task:** Finalize financial package

**Document:** 2025 Financial Statements v4

**Question:** Customer concentration methodology

This lets users move directly from:

> conversation → underlying work.

---

# 20. Secure Messaging

Messaging should be platform-controlled rather than merely an internal wrapper around email.

The system should support:

* Authenticated participants
* Encrypted transport
* Encrypted message storage
* Access-controlled conversations
* Attachment controls
* Message history
* Participant management
* Revocation handling

The platform should avoid making claims of absolute security.

---

# 21. External Participants

Professionals should be able to participate without necessarily receiving access to the entire application.

For example:

> Attorney receives secure communication invitation.

They can see:

* Their assigned matters
* Authorized conversations
* Authorized documents
* Related requests
* Their tasks

Nothing else.

This integrates the Communication Engine with:

> Identity & Access
> Consent & Access
> Stakeholder / Relationship
> Professional Review

---

# 22. Email Relationship

The platform may eventually integrate with email.

But email should not automatically become the system of record.

A message could be:

> Imported from email

or:

> Sent through platform

or:

> Referenced from an external communication.

The system should distinguish those origins.

---

# 23. External Message Capture

Users may receive important information outside the platform.

Example:

An attorney emails:

> “We need the revised employee allocation schedule.”

The owner can capture it into the platform as:

> **Request**

The system can then create the appropriate workflow.

This gives the platform a bridge between real-world communication and structured execution.

---

# 24. Message Status

Messages can have:

**Sending**

**Sent**

**Delivered**

**Read**

**Failed**

**Recalled**, where technically and contractually appropriate

The platform should distinguish:

> Delivered

from:

> Read

It should not infer that someone understood or agreed merely because they opened a message.

---

# 25. Message Editing

Editing should be supported carefully.

For example:

> Original message
>
> Edited 2:14 PM

The platform should preserve an edit history for material edits.

It should not silently rewrite historical messages.

The exact editing/retention rules can be governed by Policy / Compliance.

---

# 26. Message Deletion

Deletion should be handled carefully because communication can become transaction evidence.

Instead of silently deleting a message, the platform may support:

> Remove from active conversation

while retaining a governed historical record where policy requires.

The platform should clearly distinguish:

**Hide**

**Delete**

**Archive**

**Permanently destroy**

These are not the same action.

---

# 27. Attachments

Messages may reference documents.

But attachments should not bypass the Local Vault / Consent architecture.

Preferred flow:

> Attach from Vault

↓

> Consent & Access evaluates disclosure

↓

> Authorized document/version attached

The Communication Engine should not create an uncontrolled document repository.

---

# 28. Sensitive Messages

A message itself can be sensitive.

Example:

> “The owner is considering reducing the purchase price.”

That could contain confidential transaction information.

Therefore messages should have sensitivity metadata and appropriate access controls.

A conversation marked confidential should not automatically become visible to every transaction participant.

---

# 29. Message-Level vs Conversation-Level Access

Normally permissions should be defined at the conversation level.

But some situations may require:

> A message visible only to a subset of participants.

The architecture should support message-level restrictions without making that the default.

Otherwise communication becomes too difficult to manage.

---

# 30. Forwarding

Forwarding should not automatically grant access.

A participant should not be able to take:

> Confidential transaction conversation

and forward it to a new participant without the required authorization.

The system can instead provide:

> **Invite participant**

which invokes Identity & Access + Consent & Access.

---

# 31. Participant Changes

If a professional leaves the transaction:

> Communication access can be revoked.

Existing historical conversations remain preserved according to policy, but the former participant cannot continue participating unless reauthorized.

This separates:

> historical access

from:

> current access.

---

# 32. Communication and Revocation

Suppose an attorney's transaction access is revoked.

Communication should:

* Prevent new messages to restricted conversations
* Prevent access to current protected conversations where applicable
* Preserve historical records subject to policy
* Update participant status
* Notify affected parties where appropriate

Consent & Access determines authorization.

Communication enforces the result for conversations.

---

# 33. Questions That Become Tasks

This is an important workflow bridge.

Example:

> Lender: “Can you provide the updated 2025 debt schedule?”

Communication identifies:

> Request.

Workflow creates:

> Task: Provide updated debt schedule.

Document Readiness creates:

> Requirement: Debt schedule.

The three systems now work together without becoming one system.

---

# 34. Questions That Need Professional Review

Example:

> Owner: “Would changing the ownership percentage affect the proposed structure?”

Communication records:

> Question.

Professional Review can create:

> Review request.

The professional answers through the appropriate professional workflow.

The formal determination, if one exists, is stored separately.

---

# 35. Communication and Transaction Timeline

Important communications should optionally appear in the transaction timeline.

Example:

> **Oct 14**
>
> Lender requested updated debt schedule.

> **Oct 15**
>
> Owner provided requested schedule.

> **Oct 16**
>
> Lender confirmed receipt.

The timeline should be assembled from events and references rather than duplicated manually.

---

# 36. Communication Search

Search should support:

* Sender
* Recipient
* Conversation
* Transaction
* Date
* Topic
* Related document
* Related task
* Question
* Request
* Professional
* Keyword

Example:

> Search: “purchase price”

returns:

* Messages
* Questions
* Decisions
* Professional discussions
* Related documents
* Related transaction events

Access filtering must happen before results are shown.

---

# 37. Communication History

The platform should preserve a coherent chronology.

For example:

```text
Sept 17
Owner asks valuation question.

Sept 18
Valuation professional responds.

Sept 19
Owner uploads requested data.

Sept 22
Valuation completed.

Sept 23
Lender notified.
```

A user should be able to understand:

> **How did we get here?**

without reconstructing the story from email accounts, text messages, and scattered spreadsheets.

---

# 38. “Why Am I Seeing This?”

Every communication should be contextual.

For example:

> You're seeing this conversation because you're assigned to the Financing Review for Transaction TX-004.

This should be derived from Identity, Stakeholder, Transaction, and Consent rules.

---

# 39. “Who Can See This?”

A user should be able to inspect:

> **Conversation access**

and see:

* Owner
* CPA
* Lender

with the applicable access context.

This leverages the Consent & Access Engine.

---

# 40. Communication Notifications

Communication should emit events.

Example:

> MessageSent

> QuestionCreated

> RequestCreated

> ResponseReceived

> ConversationMentioned

The Notification Engine determines whether someone receives:

* In-app alert
* Email
* Push notification
* Digest

Communication should not implement its own notification infrastructure.

---

# 41. Notification Preferences

Users should eventually control:

* Immediate alerts
* Daily digest
* Quiet periods
* Mention-only
* Question alerts
* Request alerts

Notification preferences belong to the Notification / Identity systems.

Communication simply produces communicative events.

---

# 42. Conversation Priority

Some communications are operationally important.

A conversation or request may be labeled:

**Normal**

**Important**

**Time-sensitive**

**Blocking**

The classification should describe workflow consequences rather than emotional urgency.

Example:

> **Blocking**
>
> Response required before financing milestone can complete.

Transaction Orchestration determines the transaction impact.

---

# 43. Communication Escalation

Communication can create workflow events.

Example:

> Question unanswered for 5 business days.

Workflow may escalate.

Notification may alert the participant.

Communication records:

> Follow-up message sent.

The Communication Engine should not become the escalation engine.

---

# 44. Private Professional Conversations

Some discussions may legitimately need limited visibility.

Example:

> Owner + Attorney

The platform should support a professional-private conversation.

But it should also preserve clarity:

> **Private conversation**
>
> Participants: Owner, Attorney Smith
>
> Not visible to: Management, Lender, Employees

This matters enormously in a multi-party transaction.

---

# 45. Employee Communications

Because the platform is specifically about employee ownership, eventually there will be employee-facing communication.

Examples:

> “What does employee ownership mean?”

> “What happens to my role?”

> “When will employees be informed?”

> “What changes after closing?”

These conversations may require special access groups.

The Communication Engine should therefore not assume every participant belongs to the same audience.

---

# 46. Communication Templates

The platform may eventually provide templates for routine communications.

Examples:

> Professional request for documents

> Lender follow-up

> Stakeholder meeting summary

> Employee information request

But templates should be optional.

The platform should never automatically send substantive communications on behalf of the owner or professional without the required authorization.

---

# 47. Meeting Integration

Future integration could connect:

* Calendar
* Video meetings
* Meeting notes
* Transcripts
* Action items

A meeting could become:

> Conversation + Decisions + Tasks

But the Communication Engine should not become the meeting-recording system itself.

Those integrations can publish structured outputs into Communication and Workflow.

---

# 48. Message-to-Structure Conversion

This is potentially one of the most valuable capabilities.

A user sees:

> “The lender said they need the last three years of customer concentration data.”

The platform can offer:

> **Create request?**

Then:

> Document Readiness → create requirement.

> Workflow → create task.

> Transaction → link to financing milestone.

Communication remains the source conversation.

The structured systems create the execution machinery.

---

# 49. Professional Conversation-to-Determination Boundary

Likewise:

> Attorney message:

“Your proposed governance structure creates an issue that needs to be addressed.”

The platform should detect:

> Possible professional determination / review item.

But it should not automatically convert the statement into:

> Professional Determination = Issue confirmed.

Instead:

> **Professional review item identified.**

A professional formally determines the matter through Professional Review.

---

# 50. Communication and AI

AI can assist with conversations, but it must remain subordinate to the communication record.

Potential capabilities:

* Summarize long threads
* Identify unanswered questions
* Extract requests
* Detect action items
* Link messages to transaction objects
* Draft responses
* Identify duplicate questions
* Surface important historical context

But AI-generated interpretations must remain distinguishable from actual messages.

Example:

> **AI Summary**
>
> The lender appears to be waiting for the updated debt schedule.

versus:

> **Actual message**
>
> “Please send the updated debt schedule.”

The system must never blur the two.

---

# 51. AI Privacy

AI conversation assistance must obey the same local-first and Consent & Access principles.

The owner could have:

> **Local AI summary**

without transmitting the conversation.

Or:

> **Cloud AI summary**

with explicit disclosure of what is sent.

Again:

> The existence of AI capability does not create automatic authorization to send private communications to an AI service.

---

# 52. Conversation Summaries

Long transactions will generate thousands of messages.

The engine should eventually maintain:

### Thread Summary

> What is this discussion about?

### Open Questions

> What remains unanswered?

### Open Requests

> What remains outstanding?

### Decisions Referenced

> What decisions are connected?

### Latest Position

> What is currently being discussed?

Summaries should always be identified as generated summaries rather than original communication.

---

# 53. Transaction Communication Digest

The platform can generate a structured digest:

> **Since your last review**
>
> 3 important conversations updated.
>
> 2 questions answered.
>
> 1 lender request remains open.
>
> 1 professional issue requires your attention.

This is where Communication, Workflow, Notification, and Transaction Orchestration work together.

---

# 54. Communication Auditability

The engine should retain:

* Sender
* Recipient
* Timestamp
* Conversation
* Message
* Edit history
* Attachments/references
* Participant changes
* Visibility changes

Audit / Provenance retains the wider system-level audit trail.

Communication owns the communication history.

---

# 55. Retention

Communication records can become transaction records.

Therefore the system needs policy-driven retention options.

Examples:

* Active
* Archived
* Retained
* Restricted
* Deleted according to policy

The platform should not assume every chat message should live forever.

But it should also not casually delete potentially important transaction history.

---

# 56. Communication States

Conversation:

**Open**

**Inactive**

**Resolved**

**Archived**

**Restricted**

Messages:

**Draft**

**Sending**

**Sent**

**Delivered**

**Read**

**Failed**

Questions:

**Open**

**Waiting**

**Answered**

**Closed**

Requests:

**Open**

**Assigned**

**Waiting**

**Fulfilled**

**Declined**

**Cancelled**

These state models should remain separate.

---

# 57. Core Data Objects

## Conversation

Participants, visibility, context, subject, status.

## Message

Individual communication record.

## Thread

Parent/child message relationship.

## Question

Question requiring resolution.

## Request

Action or information requested from another party.

## Comment

Contextual note attached to an object.

## ParticipantMembership

Defines who belongs to a conversation and under what role.

## MessageReference

Link to another platform object.

## AttachmentReference

Pointer to an authorized resource without becoming its storage owner.

## ConversationSummary

Generated contextual summary.

---

# 58. Engine Contract

The engine should expose capabilities such as:

```text id="0tr4cc"
createConversation()
addParticipant()
removeParticipant()
sendMessage()
replyToMessage()
editMessage()
createQuestion()
answerQuestion()
reopenQuestion()
createRequest()
respondToRequest()
addComment()
linkObject()
searchConversations()
searchMessages()
archiveConversation()
restoreConversation()
getConversationHistory()
getOpenQuestions()
getOpenRequests()
getUnreadMessages()
```

Events emitted could include:

```text id="oe45i6"
MessageSent
MessageDelivered
MessageRead
QuestionCreated
QuestionAnswered
RequestCreated
RequestFulfilled
CommentAdded
ConversationCreated
ParticipantAdded
ParticipantRemoved
ConversationArchived
```

---

# 59. Security Rules

The Communication Engine should enforce:

**1. No participant can see a conversation merely because they belong to the transaction.**

**2. Conversation membership must correspond to authorized access.**

**3. Restricted conversations remain restricted.**

**4. Attachments cannot bypass Consent & Access.**

**5. Forwarding cannot silently grant access.**

**6. Former participants lose current access when appropriate while historical records remain governed by retention policy.**

**7. Sensitive messages inherit appropriate confidentiality controls.**

**8. AI processing does not bypass disclosure controls.**

---

# 60. Failure Handling

Messages and requests are important enough that delivery failures must be visible.

Example:

> Message failed to deliver.

The system should:

* retry where appropriate
* show current status
* preserve the unsent message
* notify the sender
* escalate when the communication is transaction-critical

A communication should never appear:

> “Sent”

when the platform knows it was not actually delivered.

---

# 61. Relationship to the Other Engines

The architecture now looks like this:

```text id="p5m1s5"
                 COMMUNICATION
                     │
        ┌────────────┼─────────────┐
        ▼            ▼             ▼
     QUESTIONS     REQUESTS     DISCUSSIONS
        │            │             │
        ▼            ▼             ▼
   PROFESSIONAL   WORKFLOW     TRANSACTION
     REVIEW       ENGINE       ORCHESTRATION
        │            │             │
        └────────────┼─────────────┘
                     │
              STRUCTURED EVENTS
                     │
       ┌─────────────┼──────────────┐
       ▼             ▼              ▼
 DOCUMENT        CONSENT         NOTIFICATION
 READINESS       & ACCESS          ENGINE
       │             │
       ▼             ▼
 LOCAL VAULT     DISCLOSURE
```

The Communication Engine sits **across the architecture**, but does not own the architecture.

---

# 62. The Most Important Product Behavior

Whenever meaningful communication happens, the platform should ask:

> **Does this need to become structured work?**

Not every message does.

But:

> “Please send me the updated tax return”

probably should create a request.

> “Can you confirm whether the valuation includes the subsidiary?”

should create a question.

> “Let's move closing to December 15.”

should potentially create a decision/review event.

> “Thanks, received.”

should simply remain communication.

This is how the platform turns messy human conversation into structured execution **without forcing every conversation into a form**.

---

# 63. Architectural Lock

These should now be treated as requirements:

**1. Communication is a standalone engine.**

**2. It manages secure conversation, not the underlying transaction work.**

**3. Messages, questions, requests, and comments are distinct objects.**

**4. Conversation visibility is explicit.**

**5. Transaction membership does not automatically grant access to every conversation.**

**6. Questions can become structured review items.**

**7. Requests can become workflow tasks and document requirements.**

**8. Conversations can reference tasks, documents, facts, scenarios, milestones, professional reviews, and decisions without owning them.**

**9. Attachments remain owned by the Local Vault / relevant document system.**

**10. Sharing attachments requires Consent & Access authorization.**

**11. Notifications remain owned by the Notification Engine.**

**12. Tasks and dependencies remain owned by Workflow / Transaction Orchestration.**

**13. Professional determinations remain owned by Professional Review.**

**14. Owner decisions remain owned by Decision Records.**

**15. Message history is preserved and version-aware.**

**16. Participant access can be revoked without destroying historical communication records.**

**17. Sensitive conversations receive appropriate access controls.**

**18. AI-generated summaries and interpretations are clearly distinguished from actual communications.**

**19. Communication may trigger structured workflow, but it does not silently become structured truth.**

**20. All significant communication events are available to Workflow and Notification through structured events.**

**21. Search respects current access permissions before returning results.**

**22. Communication records are retained according to policy rather than arbitrary chat deletion behavior.**

**23. The engine supports external stakeholders without giving them access to the entire workspace.**

**24. The engine is independently versioned, tested, and replaceable.**

---

# 64. The Deeper Role of Communication

This engine gives us something we were missing from the architecture.

Until now, the system could tell us:

> What the owner wants.

> What the business looks like.

> What the professionals determined.

> What documents exist.

> What can be shared.

> What tasks need to happen.

The Communication Engine adds:

> **What the people involved are actually saying to each other.**

That matters because real transactions are full of statements that don't fit cleanly into a form:

> “I'm concerned about the timing.”

> “The lender is asking a different question than they asked last week.”

> “The valuation professional needs clarification.”

> “The employees aren't ready for this disclosure yet.”

> “I don't understand this part of the proposal.”

The platform can capture those conversations while still turning the important pieces into structured questions, requests, tasks, decisions, and professional reviews.

That produces a powerful loop:

```text
CONVERSATION
     ↓
QUESTION / REQUEST
     ↓
STRUCTURED WORK
     ↓
EVENT
     ↓
UPDATED STATE
     ↓
NOTIFICATION
     ↓
NEW CONVERSATION
```

So the communication layer becomes the **human nervous system** of the transaction, while Workflow remains the machinery and Orchestration remains the conductor. 🧠

The architecture is now getting particularly coherent: **Communication captures human interaction; Workflow turns events into action; Orchestration organizes that action into the transaction; Consent controls disclosure; Local Vault protects the underlying information.** That separation should keep the system from becoming one giant “chat app with a checklist attached.”
