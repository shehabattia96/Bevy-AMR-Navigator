# AMR navigation while avoiding obstacles and moving humans

For this project, I am using [Bevy](https://bevy.org/), a game engine written in Rust.

The goal is to create one or more Autonomous Mobile Robots (AMR) in the scene that can navigate while avoiding obstacles and moving humans. This movement is very simple just a vector subtraction from current position to the goal, if there is an obstacle, move around it by clearing its size (privedged information from the simulation for now).

- This is being built on an M3 Pro with 18GB of RAM.

- I initialize the project using `cargo init && cargo add bevy`. I add the `DefaultPlugins` to get basic Bevy functionality.

- When I came to do my first commit, I realized I needed to check if the project builds before I commit, so I added a quick [pre-commit](./pre-commit) script with instrucitons to soft link it to `.git/hooks/pre-commit` in the file. I also chmod +x'd it so it's ready to go checked-in.

- Analytics is super important, so I am adding the `FrameTimeDiagnosticsPlugin` and `LogDiagnosticsPlugin` plugins to display FPS right away.
    - Adding some logging, profiling and auditing tools would be a good next step, but I'm not going to do that right now.
    - Maybe at some point I will add the [Inspector GUI plugin](https://docs.rs/bevy-inspector-egui/latest/bevy_inspector_egui/) for more live debugging.

- I run the project using `cargo run`. The empty window opens and we're averaging 55 FPS.

- Time to create some components. For starters these are all the ones I can think of right away, Bevy says to keep everything modular, so we'll create physics units too:
    Entities:
    - AMR
    - Obstacle (Wall, furniture, etc..)
    - Human
    Behavior:
    - Goal
    Physics:
    - Position
    - Velocity
    - Acceleration -> Maybe this one is overkill for now. Might not need it.

I create a components folder and add them in.

- Time to create some systems. 

    - First I'll focus on spawning entities. We'll worry about behaviors later. The AMR, we'll just use a cube, a human a cylinder, and obstacles will be cones. This will use the Startup Schedule.
    - Creating the startup system was tricky, I had to use these built-in parameters that were required, and kept causing build issues, but it runs! 
    ```
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    ```

- My spawners were working but were not showing up. I needed to add a camera. I added light and a ground plane for good measure. I used this example: https://bevy.org/examples/3d-rendering/3d-viewport-to-world/

- I found that the position I was supplying to my components was not being applied to the spawn transform, so I added that in too.

- Next I'll make the AMR move towards the goal. I'll use Bevy's [eventing system](https://bevy-cheatbook.github.io/programming/events.html) to broadcast a goal. I'll add a simple vector subtraction from current position to the goal. We'll ignore obstacles for now. 

I went a step further and assigned the goal by clicking the mouse. I added the nice ground circle visual from the example. The robot is moving towards a goal, just not the right one atm.


## Misc notes
- Standards are important: https://bevy-cheatbook.github.io/fundamentals/coords.html
    - Y is up/down, Z is in/out of the screen.
    - Origin top left of screen.
- Bevy quickstart has a lot of neat optimizations notes. For example, link time optimizations for release builds, and notes that dev builds could take long and to add some configs to Cargo.toml. I don't think it's necessary for now, but will follow their instructions if it starts becoming a problem.
- Schdules: Update for UI and input handling. FixedUpdate for Physics and Networking. bevy-cheatbook is the best: https://bevy-cheatbook.github.io/programming/schedules.html Edit: nevermind it's so outdated for so many tutorials.
- SRGB vs LinearRGB: https://medium.com/@Jacob_Bell/programmers-guide-to-gamma-correction-4c1d3a1724fb 