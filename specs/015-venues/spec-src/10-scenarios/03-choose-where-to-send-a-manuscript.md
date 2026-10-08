### User Story 3 - Choose where to send a manuscript (Priority: P3)

For a manuscript, a researcher lists the venues it could go to, in order of preference,
with a note on why each fits. They compare the candidates side by side — next deadline,
standing, cost, time to decision, requirements — and choose one. The manuscript then takes
that venue's deadline and requirements. If it is rejected, the next candidate is ready.

**Why this priority**: This is the decision the register exists to support. It needs venues
and their calls, and a manuscript to decide for.

**Independent Test**: Give a manuscript three candidate venues with fit notes, compare
them, aim it at the first, confirm the manuscript's target and deadline, then record that
it was not accepted and move to the second candidate.

**Acceptance Scenarios**:

1. **Given** a manuscript, **When** the researcher adds venues as candidates in order of
   preference, each with a note on its fit, **Then** the manuscript shows its shortlist.
2. **Given** a manuscript with candidates, **When** the researcher compares them, **Then**
   one table shows for each its kind, next deadline and time remaining, standing in the
   researcher's preferred ranking scheme, costs, review model, open-access policy, the
   researcher's own time to decision there, and the researcher's interest.
3. **Given** a candidate, **When** the researcher asks whether the manuscript meets its
   requirements, **Then** the tool compares the manuscript's length, kind, language, and
   anonymity with the venue's or call's requirements and reports each as met, not met, or
   not checkable.
4. **Given** a manuscript, **When** the researcher aims it at a venue, or at a particular
   call, **Then** the manuscript's target venue and deadline are set from it, and the
   manuscript appears under that call.
5. **Given** a manuscript aimed at a call, **When** the call's date changes, **Then** the
   manuscript's deadline follows.
6. **Given** a manuscript aimed at a venue marked to avoid, **When** it is aimed, **Then**
   the tool warns with the recorded reason and asks for confirmation.
7. **Given** a manuscript aimed at a venue with a template recorded, **When** the researcher
   starts its document, **Then** that template is offered.
8. **Given** a manuscript that was not accepted at its target, **When** the researcher moves
   on, **Then** the next candidate becomes the target, the earlier one is kept in the
   manuscript's history, and the shortlist shows which have been tried.
9. **Given** a manuscript's topics and kind, **When** the researcher asks for suggestions,
   **Then** venues from the register whose topics overlap, that take that kind of
   manuscript, and that are not marked to avoid are listed, those with an upcoming deadline
   first.
10. **Given** a degree programme or a funder that counts only venues of a certain standing,
    **When** the researcher records that rule, **Then** candidates that do not meet it are
    marked.
11. **Given** a venue that does not exist in the register, **When** the researcher adds it
    as a candidate by name, **Then** the tool offers to create it.
12. **Given** the same venue added twice to a shortlist, or a call whose submission date has
    passed, **When** the researcher saves or aims, **Then** the tool rejects the first and
    warns on the second.

---
