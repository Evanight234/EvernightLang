import { defineConfig } from 'astro/config';
import tailwindcss from '@tailwindcss/vite';
import remarkMdLink from './plugins/remark-md-link.mjs';

export default defineConfig({
  site: 'https://evernight-lang.org',
  markdown: {
    remarkPlugins: [remarkMdLink],
  },
  vite: {
    plugins: [tailwindcss()],
  },
});
