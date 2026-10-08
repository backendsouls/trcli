# Feature Specification: TRCLI Activity Reports

**Feature Branch**: `006-reports`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add report function spec for daily, weekly, monthly, bimonthly, half-year(better name here), custom dates"

## Overview

A researcher is regularly asked "what did you do?" — by themselves at the end of a day, by
a supervisor every week, by a programme every semester, by a funder every year. The answer
is already in the workspace: every reference added, page annotated, run finished, result
recorded, task completed, and milestone reached is on record. This specification adds the
**report**: an account of what happened in the workspace during a period of time, produced
on demand, for the period and the audience the researcher chooses.

### Report periods

| Period | Covers | Note |
|--------|--------|------|
| Daily | One calendar day | |
| Weekly | Seven days, starting on the configured first day of the week | Monday by default |
| Monthly | One calendar month | |
| Bimonthly | Two consecutive calendar months | "Bimonthly" here always means *every two months*, never twice a month |
| Quarterly | Three consecutive calendar months | Added; not in the request |
| **Semiannual** | Six consecutive calendar months | The name chosen for "half-year"; "semester" is accepted as another word for it |
| Annual | Twelve consecutive calendar months | Added; not in the request |
| Custom | Any start date to any end date | Also "the last N days" |

Periods longer than a month are counted from the first month of the researcher's year,
which is January unless they set another (for example, an academic year starting in
August), so that "the second semiannual period" means the same thing to them and to their
institution.
