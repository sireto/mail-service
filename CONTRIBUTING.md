## Contribution
We welcome contributions from developers of all skill levels! To contribute:
1. Fork the Repository
Click the "Fork" button at the top right of this page to create your own copy.

2. Clone Your Fork
    ```bash
    git clone https://github.com/<your-username>/mail-service.git
    cd mail-service
    ```

3. Create a New Branch
    ```bash
    git checkout -b feature/<your-feature-name>
    ```

4. Make Your Changes
    Implement your feature or bug fix. Be sure to follow the existing code style and naming conventions.

5. Test Your Changes
    Run the relevant tests (cargo test, yarn storybook) and ensure everything passes.

6. Commit and Push
    ```bash
    git add .
    git commit -m "Add <your feature/fix>"
    git push origin feature/<your-feature-name>
    ```

7. Open a Pull Request
    Go to the original repository on GitHub and open a pull request from your fork.

## Guidelines
- Keep pull requests small and focused.
- Write clear, descriptive commit messages.
- If your change introduces a new feature or configuration, update the documentation accordingly.
- Discuss major changes with maintainers before starting work.
- Please run `yarn lint` and ensure no ESLint errors before committing (if applicable).