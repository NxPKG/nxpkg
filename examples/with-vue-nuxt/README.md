# Nxpkgrepo VueJS/NuxtJS starter

This is an official starter Nxpkgrepo.

## Using this example

Run the following command:

```sh
npx create-nxpkg@latest -e with-vue-nuxt
```

## What's inside?

This Nxpkgrepo includes the following packages/apps:

### Apps and Packages

- `docs`: a [Nuxt](https://nuxt.com/) app
- `web`: another [Vue3](https://vuejs.org/) app
- `ui`: a stub Vue component library shared by both `web` and `docs` applications
- `eslint-config-custom`: `eslint` configurations (includes `@nuxtjs/eslint-config-typescript` and `@vue/eslint-config-typescript`)
- `tsconfig`: `tsconfig.json`s used throughout the monorepo

Each package/app is 100% [TypeScript](https://www.typescriptlang.org/).

### Utilities

This Nxpkgrepo has some additional tools already setup for you:

- [TypeScript](https://www.typescriptlang.org/) for static type checking
- [ESLint](https://eslint.org/) for code linting
- [Prettier](https://prettier.io) for code formatting

### Build

To build all apps and packages, run the following command:

```
cd my-nxpkgrepo
pnpm build
```

### Develop

To develop all apps and packages, run the following command:

```
cd my-nxpkgrepo
pnpm dev
```

### Remote Caching

Nxpkgrepo can use a technique known as [Remote Caching](https://nxpkg.build/repo/docs/core-concepts/remote-caching) to share cache artifacts across machines, enabling you to share build caches with your team and CI/CD pipelines.

By default, Nxpkgrepo will cache locally. To enable Remote Caching you will need an account with Vercel. If you don't have an account you can [create one](https://vercel.com/signup), then enter the following commands:

```
cd my-nxpkgrepo
npx nxpkg login
```

This will authenticate the Nxpkgrepo CLI with your [Vercel account](https://vercel.com/docs/concepts/personal-accounts/overview).

Next, you can link your Nxpkgrepo to your Remote Cache by running the following command from the root of your Nxpkgrepo:

```
npx nxpkg link
```

## Useful Links

Learn more about the power of Nxpkgrepo:

- [Tasks](https://nxpkg.build/repo/docs/core-concepts/monorepos/running-tasks)
- [Caching](https://nxpkg.build/repo/docs/core-concepts/caching)
- [Remote Caching](https://nxpkg.build/repo/docs/core-concepts/remote-caching)
- [Filtering](https://nxpkg.build/repo/docs/core-concepts/monorepos/filtering)
- [Configuration Options](https://nxpkg.build/repo/docs/reference/configuration)
- [CLI Usage](https://nxpkg.build/repo/docs/reference/command-line-reference)
