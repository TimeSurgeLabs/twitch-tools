---
name: repo
type: repo
agent: CodeActAgent
---

# Clean Code Guidelines

## Constants Over Magic Numbers
- Replace hard-coded values with named constants
- Use descriptive constant names that explain the value's purpose
- Keep constants at the top of the file or in a dedicated constants file

## Meaningful Names
- Variables, functions, and classes should reveal their purpose
- Names should explain why something exists and how it's used
- Avoid abbreviations unless they're universally understood

## Smart Comments
- Don't comment on what the code does - make the code self-documenting
- Use comments to explain why something is done a certain way
- Document APIs, complex algorithms, and non-obvious side effects

## Single Responsibility
- Each function should do exactly one thing
- Functions should be small and focused
- If a function needs a comment to explain what it does, it should be split

## DRY (Don't Repeat Yourself)
- Extract repeated code into reusable functions
- Share common logic through proper abstraction
- Maintain single sources of truth

## Clean Structure
- Keep related code together
- Organize code in a logical hierarchy
- Use consistent file and folder naming conventions

## Encapsulation
- Hide implementation details
- Expose clear interfaces
- Move nested conditionals into well-named functions

## Code Quality Maintenance
- Refactor continuously
- Fix technical debt early
- Leave code cleaner than you found it

## Testing
- Write tests before fixing bugs
- Keep tests readable and maintainable
- Test edge cases and error conditions

## Version Control
- Write clear commit messages
- Make small, focused commits
- Use meaningful branch names 

# Code Quality Guidelines

## Verify Information
Always verify information before presenting it. Do not make assumptions or speculate without clear evidence.

## File-by-File Changes
Make changes file by file and give me a chance to spot mistakes.

## No Apologies
Never use apologies.

## No Understanding Feedback
Avoid giving feedback about understanding in comments or documentation.

## No Whitespace Suggestions
Don't suggest whitespace changes.

## No Summaries
Don't summarize changes made.

## No Inventions
Don't invent changes other than what's explicitly requested.

## No Unnecessary Confirmations
Don't ask for confirmation of information already provided in the context.

## Preserve Existing Code
Don't remove unrelated code or functionalities. Pay attention to preserving existing structures.

## Single Chunk Edits
Provide all edits in a single chunk instead of multiple-step instructions or explanations for the same file.

## No Implementation Checks
Don't ask the user to verify implementations that are visible in the provided context.

## No Unnecessary Updates
Don't suggest updates or changes to files when there are no actual modifications needed.

## Provide Real File Links
Always provide links to the real files, not x.md.

## No Current Implementation
Don't show or discuss the current implementation unless specifically requested.


# React Best Practices

## Component Structure
- Use functional components over class components
- Keep components small and focused
- Extract reusable logic into custom hooks
- Use composition over inheritance
- Implement proper prop types with TypeScript
- Split large components into smaller, focused ones

## Hooks
- Follow the Rules of Hooks
- Use custom hooks for reusable logic
- Keep hooks focused and simple
- Use appropriate dependency arrays in useEffect
- Implement cleanup in useEffect when needed
- Avoid nested hooks

## State Management
- Use useState for local component state
- Implement useReducer for complex state logic
- Use Context API for shared state
- Keep state as close to where it's used as possible
- Avoid prop drilling through proper state management
- Use state management libraries only when necessary

## Performance
- Implement proper memoization (useMemo, useCallback)
- Use React.memo for expensive components
- Avoid unnecessary re-renders
- Implement proper lazy loading
- Use proper key props in lists
- Profile and optimize render performance

## Forms
- Use controlled components for form inputs
- Implement proper form validation
- Handle form submission states properly
- Show appropriate loading and error states
- Use form libraries for complex forms
- Implement proper accessibility for forms

## Error Handling
- Implement Error Boundaries
- Handle async errors properly
- Show user-friendly error messages
- Implement proper fallback UI
- Log errors appropriately
- Handle edge cases gracefully

## Testing
- Write unit tests for components
- Implement integration tests for complex flows
- Use React Testing Library
- Test user interactions
- Test error scenarios
- Implement proper mock data

## Accessibility
- Use semantic HTML elements
- Implement proper ARIA attributes
- Ensure keyboard navigation
- Test with screen readers
- Handle focus management
- Provide proper alt text for images

## Code Organization
- Group related components together
- Use proper file naming conventions
- Implement proper directory structure
- Keep styles close to components
- Use proper imports/exports
- Document complex component logic 

Repository: MyProject
Description: A web application for task management

Directory Structure:
- src/: Main application code
- tests/: Test files
- docs/: Documentation

Setup:
- Run `npm install` to install dependencies
- Use `npm run dev` for development
- Run `npm test` for testing

Guidelines:
- Follow ESLint configuration
- Write tests for all new features
- Use TypeScript for new code

If adding a new component in src/components, always add appropriate unit tests in tests/components/.


# Tauri 2.0 Best Practices

## Project Structure
- **Separate Frontend and Backend**: Organize your project with a clear distinction between frontend (webview) and backend (Rust) code.
- **Frontend Directory**: Place frontend code in a `src` directory (or a similar convention based on your frontend framework, e.g., React, Vue).
  - Use subdirectories for route-specific components if using a router (e.g., `src/routes`).
  - Place shared components in `src/components`.
  - Place utilities and helpers in `src/lib`.
- **Backend Directory**: Place Rust backend code in the `src-tauri` directory.
  - Follow Rust’s module system for organizing backend code (e.g., `src-tauri/src/main.rs`, `src-tauri/src/lib.rs`).
- **Naming Convention**: Use lowercase with dashes for directories (e.g., `components/auth-wizard`).

## Components
- **Efficiency**: Design components to minimize resource usage, as Tauri apps run on desktop environments with varying hardware capabilities.
- **Backend Interaction**: Use Tauri’s API (`invoke`) to call Rust backend functions from the frontend.
- **Error Handling**: Implement proper error handling for backend calls to ensure a robust user experience.
- **Asynchronous Operations**: Consider using Suspense or similar patterns in your frontend framework for handling asynchronous backend interactions.
- **File Organization**: Place static content (e.g., constants) and interfaces at the end of files for better readability.

## Performance
- **Asset Optimization**: Optimize images and assets by using appropriate formats (e.g., WebP), compressing them, and implementing lazy loading when possible.
- **Minimize Frontend Load**: Offload heavy computations to the Rust backend instead of the frontend to reduce resource demands.
- **State Efficiency**: Use efficient state management to avoid unnecessary re-renders in the frontend (e.g., React’s `useMemo`, `useCallback`).
- **Lazy Loading**: Implement lazy loading for non-critical components or features to improve startup time.
- **Resource Monitoring**: Monitor and optimize the app’s memory and CPU usage, leveraging tools like Tauri’s debugging capabilities.

## Data Fetching
- **Local Data**: Use Tauri’s file system APIs (e.g., `@tauri-apps/api/fs`) for accessing local files or databases on the user’s machine.
- **Error Handling**: Implement proper error handling for all data operations, including file access and network requests.
- **Caching**: Use caching strategies for frequently accessed data to reduce redundant operations.
- **UI Feedback**: Handle loading and error states in the UI to keep users informed.
- **Remote Data**: For network requests, handle offline scenarios and slow connections gracefully, providing fallback options where applicable.

## Routing
- **Frontend Router**: Follow best practices for your chosen frontend router (e.g., React Router for React-based Tauri apps), as Tauri itself does not provide routing.
- **Route States**: Implement proper loading and error states for route transitions to enhance user experience.
- **Dynamic Routes**: Use dynamic routing appropriately, ensuring flexibility and maintainability.
- **Security**: Ensure routes are secure and handle permissions if they expose sensitive functionality.

## Forms and Validation
- **Validation Libraries**: Use libraries like Zod for form validation in the frontend, ensuring data integrity.
- **Backend Validation**: If forms submit data to a backend (local or remote), implement server-side validation in Rust for additional security.
- **Error Handling**: Handle form errors gracefully, providing clear feedback to users.
- **Loading States**: Show loading states during form submissions or data processing to indicate progress.

## State Management
- **Frontend State**: Use state management solutions suited to your frontend framework (e.g., React Context, Redux).
- **Minimize Client State**: Keep client-side state minimal, leveraging the Rust backend for complex state logic when possible.
- **Tauri Features**: Utilize Tauri’s state management or event system (e.g., `@tauri-apps/api/event`) for coordination between frontend and backend.
- **Synchronization**: Ensure proper synchronization between frontend and backend state to maintain consistency.
- **Loading States**: Handle loading states for asynchronous state updates to improve responsiveness.

## Security
- **Input Sanitization**: Ensure the Rust backend sanitizes all inputs to prevent injection attacks or unexpected behavior.
- **Capabilities System**: Use Tauri’s capabilities system to define and restrict what the frontend can access in the backend.
- **Dependency Management**: Keep all dependencies (Rust crates, npm packages) up to date to mitigate known vulnerabilities.
- **Error Handling**: Implement error handling that avoids leaking sensitive information to the user.

## Cross-Platform Development
- **Testing**: Test the app on all target platforms (Windows, macOS, Linux) to ensure consistent behavior.
- **Platform Differences**: Handle platform-specific differences in UI or functionality (e.g., file system paths, native dialogs).
- **Platform Detection**: Use Tauri’s platform detection features (e.g., `@tauri-apps/api/os`) when conditional logic is needed.
- **Consistency**: Ensure file paths and system interactions work correctly across platforms.

## Bundling and Distribution
- **Configuration**: Configure the bundler in `tauri.conf.json` appropriately for each platform, specifying icons, identifiers, and other settings.
- **App Signing**: Sign the app for platforms that require it (e.g., macOS) to ensure trust and compatibility.
- **Updates**: Implement an update mechanism using Tauri’s updater if the app needs to check for new versions.
- **Resource Inclusion**: Ensure all necessary resources (e.g., assets, frontend bundles) are properly included in the final bundle.


# TypeScript Best Practices

## Type System
- Prefer interfaces over types for object definitions
- Use type for unions, intersections, and mapped types
- Avoid using `any`, prefer `unknown` for unknown types
- Use strict TypeScript configuration
- Leverage TypeScript's built-in utility types
- Use generics for reusable type patterns

## Naming Conventions
- Use PascalCase for type names and interfaces
- Use camelCase for variables and functions
- Use UPPER_CASE for constants
- Use descriptive names with auxiliary verbs (e.g., isLoading, hasError)
- Prefix interfaces for React props with 'Props' (e.g., ButtonProps)

## Code Organization
- Keep type definitions close to where they're used
- Export types and interfaces from dedicated type files when shared
- Use barrel exports (index.ts) for organizing exports
- Place shared types in a `types` directory
- Co-locate component props with their components

## Functions
- Use explicit return types for public functions
- Use arrow functions for callbacks and methods
- Implement proper error handling with custom error types
- Use function overloads for complex type scenarios
- Prefer async/await over Promises

## Best Practices
- Enable strict mode in tsconfig.json
- Use readonly for immutable properties
- Leverage discriminated unions for type safety
- Use type guards for runtime type checking
- Implement proper null checking
- Avoid type assertions unless necessary

## Error Handling
- Create custom error types for domain-specific errors
- Use Result types for operations that can fail
- Implement proper error boundaries
- Use try-catch blocks with typed catch clauses
- Handle Promise rejections properly

## Patterns
- Use the Builder pattern for complex object creation
- Implement the Repository pattern for data access
- Use the Factory pattern for object creation
- Leverage dependency injection
- Use the Module pattern for encapsulation 

