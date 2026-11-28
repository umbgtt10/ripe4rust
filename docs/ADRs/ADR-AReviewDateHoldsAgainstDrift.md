# ADR-AReviewDateHoldsAgainstDrift

- **Status:** Accepted
- **Date:** 2026-11-28

## Context

The rule against drift asked a unit that a pin held (rule 4) as overripe when the
workspace was two lines or more behind, or the line in use was stale while a
newer one was active. Its purpose: rule 4 must not become "never". But the first
pins written in a real round showed the cost. The owner answered two asks, "wait"
and "later", and a pin with a review date fourteen days out recorded each answer.
Both crates were overripe, so the next plan asked both again, now as keep-pinned
or unpin. The answer was already in the pin, and every plan until the review date
would ask it again.

A pin's review date already prevents "never": on that date rule 5 asks the pin
again, whatever else holds. Only a pin released by a condition alone -- a crate's
release, or words -- has no date that comes back to it. Such a pin can wait for
ever.

## Decision

The rule against drift no longer overrides a pin hold whose pins each carry a
review date. Until that date the unit is ripening, however far behind it falls;
on that date, rule 5 asks it. A pin hold with any pin lacking a review date is
still asked overripe, as is a young line (rule 9), which no person decided.

## Rejected alternatives

- **Drift overrides every pin, as before.** That asks a decision already made and
  recorded, with its date, on every plan until that date.
- **Drift never overrides a pin.** A pin held by a release that never comes would
  hold for ever, the case the rule exists for.
- **Cap the review date** (for instance, at the hold days) and override pins
  beyond it. That second-guesses a date the owner chose; a far review date is
  the owner's word, written in the repository and seen in every plan's reasons.

## Consequences

A pinned overripe unit is asked as keep-pinned or unpin only when its pins lack a
review date. The plan still lists a held unit's reasons -- the pin's reason and
its review date -- so the hold is never silent. `hold` writes a review date by
default (the hold days after the plan's date), so a pin written through the round
holds until its date.
