import type { Config } from "tailwindcss";
const config: Config = {
  content: {
    files: ["*.html", "./src/**/*.rs", "./preline/*.js"],
  },
  theme: {
    extend: {},
  },
  plugins: [require("preline/plugin")],
};

export default config;
