# Bevy Maze

This is a project built to teach myself how to make games with Bevy. My
intention is to build a 2D adventure game which has a feature set that could be
the basis for such a game while remaining small to optimize its
understandability and potential for reuse in future games.

# Play It!

[Bevy Maze can be found on itch.io](https://jfeaver.itch.io/bevy-maze).

# Deployment

1. Tag releases in Git with the version number (i.e. vX.Y.Z) after a commit that
   updates the version number in Cargo.toml.
2. Merge the branch to `main` using a pull request to trigger CI workflows.
3. Trigger a release through GitHub by navigating to Actions > Release > Run
   workflow and enter the same version number as in #1.

This project was generated using the
[Bevy New 2D](https://github.com/TheBevyFlock/bevy_new_2d) template. Check out its
[documentation](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/README.md)
for more details on deployment and other aspects of this project.
