# Current Bugs & Issues

## UI Testing Setup Issues

### Problem
Unable to properly initialize UI tests due to shader asset dependencies in bevy_egui. The tests fail because they require proper initialization of shader assets, which is challenging in a test environment.

### Attempted Solutions
1. **Direct Asset Initialization**
   - Tried: `Assets::<Shader>::default()`
   - Failed: `Assets` doesn't implement `Default`
   - Reason: Assets require proper initialization with a channel sender

2. **Manual Asset Creation**
   - Tried: `Assets::new()`
   - Failed: Constructor is private
   - Reason: Bevy's asset system is designed to be initialized through the engine

3. **Asset Server Approach**
   - Tried: Using `AssetServer` with file-based assets
   - Failed: Couldn't initialize without full render pipeline
   - Reason: Asset server requires proper IO setup

4. **Resource Initialization**
   - Tried: `init_resource::<Assets<Shader>>()`
   - Failed: Missing `FromWorld` implementation
   - Reason: Assets aren't designed to be created from scratch

5. **Render App Integration**
   - Tried: Using `RenderApp` to initialize shader assets
   - Failed: Still can't access private constructors
   - Reason: Bevy's render system is tightly coupled

### Current Status
- Core game logic tests are passing
- UI tests are failing due to missing shader assets
- Need to find a way to either:
  1. Mock the shader assets properly
  2. Use Bevy's test utilities (if they exist)
  3. Create custom test plugins
  4. Skip UI tests requiring shader assets
  5. Restructure UI tests to avoid needing full egui setup

### Impact
- Cannot run UI tests in CI/CD pipeline
- Manual testing required for UI components
- Reduced confidence in UI code changes

### Next Steps
1. Research how Bevy's own UI tests handle this
2. Consider creating mock implementations of required resources
3. Look into headless testing options
4. Consider restructuring UI to be more testable
5. Document workarounds for manual testing

### Related Components
- `src/plugins/ui/test_utils.rs`
- `src/plugins/ui/graph_view.rs`
- `src/plugins/ui/system_status.rs`
- `src/plugins/ui/planning_panel.rs`

### Notes
- This is a common issue in Bevy ecosystem when testing UI components
- The tight coupling between egui and shader assets makes testing challenging
- Need to balance between test coverage and practical testing approaches

🎨 Easter egg: "Even the most beautiful UI needs its shaders to shine" 