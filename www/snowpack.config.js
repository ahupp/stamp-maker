// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  workspaceRoot: "../",
  alias: {
    "stamp-maker-wasm": "../wasm/pkg",
  },
  mount: {
    "../wasm/pkg": {
      url: "/dist/stamp-maker-wasm",
      static: true,
      resolve: true,
    },
    "./": {
      url: "/",
    },
  },
  plugins: [
    /* ... */
  ],
  packageOptions: {
    /* ... */
  },
  devOptions: {
    /* ... */
  },
  buildOptions: {
    /* ... */
  },
};
