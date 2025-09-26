# AMR navigation while avoiding obstacles and humans

- This project uses [Bevy](https://bevy.org/), a game engine written in Rust.

- The goal of this project is to create one or more Autonomous Mobile Robots (AMR) in the scene that can navigate while avoiding obstacles and humans. 

- Path planning is a very simple vector subtraction from current position to the goal. If there is an obstacle within a specified range of the AMR, it will attempt to move around it by moving in a vector perpendicular to the obstacle.

- The user can click anywhere in the scene to set a new goal for the AMR. The goal is broadcast using Bevy's eventing system.

- This is being developed on an M3 Pro with 18GB of RAM.

- I initialized the project using `cargo init && cargo add bevy`. I add the `DefaultPlugins` to get basic Bevy functionality.

- When I was about to make my first commit, I realized I needed to check if the project builds before I commit, so I added a quick [pre-commit](./pre-commit) script with instrucitons to soft link it to `.git/hooks/pre-commit` in the file. I also chmod +x'd it so it's ready to go checked-in.

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

- Time to create some systems:

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

- I went a step further and assigned the goal by clicking the mouse. I added the nice ground circle visual from the example I linked above. The robot is moving towards a goal, just not the right one atm. In retrospect, the mouse click right now uses window position im pixels, which needs to be converted to world position.

- The last step is to implement collision detection and avoidance. When an object is a certain distance from the AMR, we'll apply a simple avoidance vector by taking the direction to the object and creating a vector perpendicular to it. We'll also ignore applying that avoidance vector if the obstacle is behind us, otherwise we get stuck adjacent to the object we're trying to avoid.

After implementing it, it works decently for how simple it is. Here's a [video](./human_avoidance.mp4):

<video src="./human_avoidance.mp4" ></video>

## Future work

- Implement obstacles such as walls, furniture, etc...
- Improve path planning so that it's not so simple. Right now collision avoidance iterates over every known human or object and this is expensive, maybe a structure like a kdtree would be better.
- Make the humans move around on their own.
- Spawn thousands of AMR's and Humans and see how it performs.
- I would also like to learn more about Bevy traits, I think there are many that I still have not used.

## Misc notes
- Standards are important: https://bevy-cheatbook.github.io/fundamentals/coords.html
    - Y is up/down, Z is in/out of the screen.
    - Origin top left of screen.
- Bevy quickstart has a lot of neat optimizations notes. For example, link time optimizations for release builds, and notes that dev builds could take long and to add some configs to Cargo.toml. I don't think it's necessary for now, but will follow their instructions if it starts becoming a problem.
- Schdules: Update for UI and input handling. FixedUpdate for Physics and Networking. bevy-cheatbook is the best: https://bevy-cheatbook.github.io/programming/schedules.html Edit: nevermind it's so outdated for so many tutorials.
- SRGB vs LinearRGB: https://medium.com/@Jacob_Bell/programmers-guide-to-gamma-correction-4c1d3a1724fb 