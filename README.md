# Bevy Maze

This is a project built to teach myself how to make games with Bevy. My
intention is to build a 2D adventure game which has a feature set that could be
the basis for such a game while remaining small to optimize its
understandability and potential for reuse in future games.

# Play It!

[Bevy Maze can be found on itch.io](https://jfeaver.itch.io/bevy-maze).

# Deployment

1. Write features on feature branches and create pull requests when finished.
   Add a commit when you're close to the end that bumps the version number in
   Cargo.toml.
2. Merging the branch to `main` through a pull request triggers CI workflows.
   Get them to pass, as needed.
3. Trigger a release through GitHub by navigating to Actions > Release > Run
   workflow and enter the same version number as in #1. This also creates a
   Git tag at the latest commit matching the version number.

This project was generated using the
[Bevy New 2D](https://github.com/TheBevyFlock/bevy_new_2d) template. Check out its
[documentation](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/README.md)
for more details on deployment and other aspects of this project.
