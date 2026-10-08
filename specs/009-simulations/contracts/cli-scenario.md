# Contract: `trcli scenario`

**Spec**: [Simulations](../spec.md) — User Story 2; FR-009 to FR-017

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `scn` — a scenario may also be named `<experiment-ref>/<name>`

## Synopsis

```text
trcli scenario add <experiment-ref> --name <text> [--from <scenario>] [--set <input>=<value>]...
                   [--horizon <n>] [--warm-up <n>] [--time-step <n>] [--replications <n>]
                   [--baseline] [--description <text>]
trcli scenario list <experiment-ref>
trcli scenario show <scenario>
trcli scenario edit <scenario> [--set <input>=<value>]... [--unset <input>]... [--horizon <n>]
                    [--warm-up <n>] [--time-step <n>] [--replications <n>] [--description <text>]
trcli scenario baseline <scenario>
trcli scenario resolve <experiment-ref>
trcli scenario rm <scenario>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `scenario add` | Defines a scenario. Inputs not set take the model's default, or with `--from` the value of the scenario it is derived from | 2 on a value outside what the model allows, of the wrong kind, or for an input the model does not have; on a warm-up not shorter than the horizon; on fewer than one replication; on a name already used in the experiment — all reported together |
| `scenario add` (same values as another) | Accepted, with a warning naming the other scenario | 0 |
| `scenario list` | Each scenario with how it differs from the baseline, its horizon, its replications planned and run | 0 with a message when the experiment has no model |
| `scenario show` | Every input with its value, unit, and whether the value is the model's default, inherited, or set here | 3 when not found |
| `scenario edit` | Changes what is given; `--unset` returns an input to its inherited or default value. Names derived scenarios that will follow the change. Warns when replications exist: they keep what they recorded and are marked as an earlier definition | 5 `confirmation_required` when derived scenarios or existing replications are affected |
| `scenario baseline` | Makes this scenario the baseline; the previous one is no longer | — |
| `scenario resolve` | After a move to a new model version: lists scenarios that set an input which no longer exists or no longer allows their value, and asks what to do for each | 6 `check_failed` while any remains unresolved and it cannot ask |
| `scenario rm` | Removes a scenario; scenarios derived from it keep their values | 5 `blocked_by_dependents` when it has replications; 5 `confirmation_required` otherwise |

## Values

| Value | Rule |
|-------|------|
| `--name` | 1–100 characters of `a-z 0-9 - _`, unique within the experiment |
| `--from` | a scenario of the same experiment; derivation must not form a loop |
| `--horizon`, `--warm-up`, `--time-step` | in the model's unit of simulated time; horizon > 0, 0 ≤ warm-up < horizon, time step > 0; not asked for a model without time |
| `--replications` | whole number ≥ 1; default 1; must be 1 for a model that uses no random numbers |
| a scenario as a condition | wherever `specs/004-experiments` accepts `--condition <name>`, a scenario name is accepted |

## Example

```console
$ trcli scenario add exp-7m2c --name two-servers --from baseline --set servers=2 --replications 30
Added scn-5k1t "two-servers" (from baseline; differs in: servers 1 → 2)

$ trcli scenario list exp-7m2c
NAME           DIFFERS FROM BASELINE        HORIZON   WARM-UP   REPLICATIONS
baseline ★     —                            10000 s   1000 s    30 planned · 30 run
two-servers    servers 1 → 2                10000 s   1000 s    30 planned ·  0 run
rush-hour      arrival_rate 0.8 → 0.96      10000 s   1000 s    30 planned ·  0 run
```
