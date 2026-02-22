# Cryptfall — Claude Code Session Quick Reference

## How to use these plans

Each phase document contains **numbered sessions** (e.g., Session 0.1, 0.2, etc.). Each session has:

1. **A Claude Code Prompt** — copy-paste this into Claude Code at the start of the session
2. **Success Criteria** — checkboxes to verify before moving to the next session
3. **Technical Notes** — gotchas and tips specific to that session

## Workflow per session

```
1. Open Claude Code
2. Copy the session's prompt (including the role preamble)
3. Point Claude Code at the relevant files: "Look at crates/engine/src/"
4. Work through the implementation
5. Test against success criteria
6. If something doesn't work, stay in the same session (don't skip ahead)
7. When all criteria pass, commit and move to next session
```

## Session Index

### Phase 0: Foundation (Weeks 1–3)
| Session | Focus | Role | ~Time |
|---------|-------|------|-------|
| 0.1 | Project scaffold + raw terminal | Engine Architect | 2-3h |
| 0.2 | FrameBuffer + half-block rendering | Engine Architect | 3-4h |
| 0.3 | Double buffering + diff rendering | Engine Architect | 2-3h |
| 0.4 | Input system with held-key inference | Engine Architect | 2-3h |
| 0.5 | Fixed timestep game loop | Engine Architect | 2-3h |
| 0.6 | Integration test + review | QA Lead | 2h |

### Phase 1: Sprite Engine (Weeks 4–6)
| Session | Focus | Role | ~Time |
|---------|-------|------|-------|
| 1.1 | Sprite data format + blitting | Engine Architect | 3h |
| 1.2 | Animation system | Engine Architect | 3h |
| 1.3 | Tile map + collision | Engine Architect | 3-4h |
| 1.4 | Camera system + screen shake | Engine Architect | 2-3h |
| 1.5 | Player sprite sheet (all animations) | Visual Designer | 3-4h |
| 1.6 | Integration test + demo mode | QA Lead | 2h |

### Phase 2: Combat Core (Weeks 7–11)
| Session | Focus | Role | ~Time |
|---------|-------|------|-------|
| 2.1 | Hitbox system + basic attack | Game Designer | 3-4h |
| 2.2 | Hit feedback (pause, shake, knockback) | Game Designer | 3-4h |
| 2.3 | Particle system (5 effect types) | Engine Architect | 3-4h |
| 2.4 | Skeleton warrior AI | Game Designer | 3-4h |
| 2.5 | Ghost mage AI + projectiles | Game Designer | 3-4h |
| 2.6 | Player health + damage + death | Game Designer | 3h |
| 2.7 | Combat integration + polish | QA Lead | 3h |

### Phase 3: Dungeon Structure (Weeks 12–16)
| Session | Focus | Role | ~Time |
|---------|-------|------|-------|
| 3.1 | Room templates + data format | Game Designer | 3h |
| 3.2 | Procedural floor generator | Game Designer | 4h |
| 3.3 | Room transitions (fade, doors) | Engine Architect | 3-4h |
| 3.4 | Wave spawning + encounters | Game Designer | 3h |
| 3.5 | Minimap + floor navigation | Engine Architect | 3h |
| 3.6 | Bone King boss fight | Game Designer | 4-5h |
| 3.7 | Full run integration test | QA Lead | 3h |

### Phase 4: Progression & Depth (Weeks 17–22)
| Session | Focus | Role | ~Time |
|---------|-------|------|-------|
| 4.1 | 20+ boons + selection UI | Game Designer | 4-5h |
| 4.2 | 3 weapon types | Game Designer | 3-4h |
| 4.3 | Meta-progression + save system | Game Designer | 3-4h |
| 4.4 | Title screen + full game flow | Visual Designer | 3-4h |
| 4.5 | Balance pass (5+ runs) | QA Lead | 4-5h |

### Phase 5: SSH & Distribution (Weeks 23–26)
| Session | Focus | Role | ~Time |
|---------|-------|------|-------|
| 5.1 | SSH server foundation | Infra Engineer | 4-5h |
| 5.2 | SSH polish + save persistence | Infra Engineer | 3h |
| 5.3 | CI/CD + Docker + cross-compile | Infra Engineer | 3h |
| 5.4 | README + GIF + launch materials | Visual Designer | 3-4h |
| 5.5 | Deployment + launch checklist | Infra/QA | 3-4h |

**Total: ~32 sessions, ~100-120 hours**
At 15-25 hrs/week → **roughly 5-7 months** (with some buffer for debugging and iteration).

## Role Preamble Cheat Sheet

Copy-paste the appropriate preamble at the start of each Claude Code session:

### Engine Architect
> You are the Engine Architect for Cryptfall, a terminal game engine built on crossterm. Focus on: frame buffer, game loop, input pipeline, render pipeline, sprite/particle systems. Write zero-allocation hot paths. Design APIs for other roles. Never write game logic.

### Game Designer
> You are the Game Designer for Cryptfall, a Hades-inspired roguelike. Design combat, enemy AI, abilities, room encounters, progression. Think in game feel: hit-pause, screen shake, i-frames, attack windows. Write game logic using engine APIs. Never modify the engine directly.

### Visual Designer
> You are the Visual Designer for Cryptfall. Half-block pixel renderer, NES aesthetic. Each pixel = Unicode ▄ with true color. Design sprites as RGB arrays, animations, particles, UI. Think chunky readable silhouettes, 3-4 colors, high contrast. Output as Rust const arrays.

### Infrastructure Engineer
> You are the Infrastructure Engineer for Cryptfall. Handle SSH server (russh), CI/CD (GitHub Actions), Docker, cross-platform builds, deployment. Ensure `ssh play.cryptfall.dev` works. Focus on reliability, session management, bandwidth optimization.

### QA Lead
> You are the QA Lead for Cryptfall. Play-test, review code for edge cases, check terminal compatibility, profile performance, tune game feel. Be brutally honest about what doesn't work. Check: flickering, input lag, crashes, memory leaks, balance issues.

## Cross-Role Communication Files

Keep these files updated — they're how "roles" coordinate between sessions:

```
docs/
├── engine-requests.md    # Game Designer → Engine Architect
├── art-requests.md       # Game Designer → Visual Designer  
├── known-bugs.md         # QA → all roles
├── design-decisions.md   # Architecture Decision Records
├── tuning-values.md      # All game constants in one place
└── playtest-notes.md     # QA observations
```

## Git Workflow

```bash
# Feature branches per session
git checkout -b phase0/session-0.1-scaffold
# ... work ...
git add -A && git commit -m "Phase 0.1: project scaffold + raw terminal"
git checkout main && git merge phase0/session-0.1-scaffold

# Tag at phase completion
git tag -a v0.0.1-phase0 -m "Phase 0 complete: engine foundation"
```

## Emergency Reference

**Terminal broken after crash:**
```bash
reset       # or
stty sane   # or
tput reset
```

**Rust won't compile — lifetime issues:**
> Tell Claude Code: "I'm getting lifetime errors. The ownership model for this struct is: [describe who owns what]. Help me restructure."

**Game feels bad but I can't articulate why:**
> Switch to QA role. Say: "Play through this combat sequence and tell me specifically what feels wrong. Compare to Hades/Hollow Knight game feel."

**Lost motivation:**
> Record a GIF of whatever you have working. Watch it. Remember: nobody has EVER built a real-time action game in a terminal before. You're making something new.
