import type { Config } from "tailwindcss";
const config: Config = {
  content: {
    files: ["*.html", "./src/**/*.rs", "node_modules/preline/dist/*.js"],
  },
  theme: {
    extend: {},
  },
  plugins: [require("preline/plugin")],
};

export default config;
