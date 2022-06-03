import adapter from "@sveltejs/adapter-static";
import preprocess from "svelte-preprocess";
import path from "path";
import autoprefixer from "autoprefixer";
import tailwindcss from "tailwindcss";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  extensions: [".svelte"],

  kit: {
    // Default SvelteKit options
    // target: "#svelte",
    adapter: adapter(),

    alias: {
      $styles: path.resolve("src/lib/styles"),
      $components: path.resolve("src/lib/components"),
      $posts: path.resolve("src/posts"),
    },
  },

  preprocess: preprocess({
    postcss: true,
  }),
};

export default config;
