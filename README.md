# DevOps Entropy - A Solitaire Simulation Game 🎮

## Overview
DevOps Entropy is a single-player card game where you play as a DevOps engineer managing complex systems through probability distributions. Instead of directly controlling outcomes, you play cards that influence various distributions, creating cascading effects through your technical ecosystem.

## Core Mechanics

### 🎲 The Three Distributions

1. **Entropy Distribution**
   - Represents the natural tendency of systems towards chaos
   - Determines frequency and severity of incidents
   - Can never be eliminated, only managed

2. **Behavior Distribution**
   - Represents how your systems actually perform
   - Affected by your cards and entropy
   - Has multiple dimensions (reliability, performance, security)

3. **Customer Distribution**
   - Represents user behavior and satisfaction
   - Interacts with your behavior distribution to generate revenue
   - Creates feedback loops that affect other distributions

### 🃏 Card Play
Each turn, you can play one DevOps card that modifies these distributions. Examples:
- Deploy Monitoring (narrows entropy distribution)
- Add Redundancy (shifts behavior distribution toward reliability)
- Automate Tests (reduces variance in behavior distribution)
- Refactor System (improves entropy resistance)

### 📈 Simulation Phase
After playing a card:
1. Customer distribution interacts with behavior distribution
2. Revenue is generated (or lost)
3. Entropy distribution is sampled for incidents
4. Behavior distribution naturally drifts towards chaos
5. Feedback loops propagate changes through the system

## Sprint Structure

Each game consists of multiple sprints. During a sprint:
1. Play cards to influence distributions
2. Watch the simulation unfold
3. Receive sprint report with metrics
4. Plan next sprint based on outcomes

## Victory Conditions
- Survive X sprints while maintaining profitability
- Achieve specific reliability targets
- Build a self-sustaining system (positive feedback loops > entropy)

## Strategic Depth

### Risk Management
- Balance short-term stability vs long-term improvements
- Manage technical debt through distribution shapes
- Create positive feedback loops while containing negative ones

### Emergence
- Simple distribution interactions create complex behaviors
- Small changes can cascade into major effects
- Systems exhibit realistic DevOps patterns

## Technical Implementation
Built with 🦀 Rust, leveraging:
- `bevy` (0.11) for game engine and ECS
- `bevy_egui` (0.21) for immediate mode GUI
- `petgraph` (0.6) for system architecture simulation
- `statrs` (0.16) and `rand_distr` (0.4) for probability distributions
- `serde` (1.0) for data serialization

### Current Features
- System architecture simulation using directed graphs
- Real-time distribution sampling and visualization
- Technical debt propagation system
- Sprint-based gameplay loop
- Resource management (money, reputation)
- Multiple architecture types (starting with Monolith)

### Game States
1. **Loading** - Initial game setup
2. **Planning** - Card selection and strategy phase
3. **Running** - Simulation and outcome calculation

### System Components
- **Nodes**: Represent services/components with health and tech debt metrics
- **Edges**: Represent dependencies with reliability and latency properties
- **Distributions**: Normal and LogNormal distributions for various system behaviors

## Development Status
🚧 Alpha Development
- ✅ Core game loop implemented
- ✅ Basic system simulation
- ✅ Technical debt mechanics
- ✅ Resource management
- 🏗️ Card system (In Progress)
- 🏗️ UI Implementation (In Progress)
- 📋 Distribution visualization (Planned)

## Easter Eggs 🥚
Keep an eye out for:
- The mythical "coffee_machine" service
- Hidden test assertions about perfect systems
- Love notes from the developers in the game loop
- Quantum-entangled bugs that defy normal debugging

## Contributing
We welcome contributions! Current focus areas:
- Card system implementation
- UI/UX improvements
- Distribution visualization
- Additional architecture types
- Test coverage and property-based testing

## Building and Running
```bash
cargo run  # For development build
cargo run --release  # For optimized release build
```

## License
MIT

---
*"In theory, there is no difference between theory and practice. In practice, there is." - Commonly attributed to Yogi Berra*

*"Every line of code is a tiny prayer to the chaos gods" - DevOps Entropy Development Team*