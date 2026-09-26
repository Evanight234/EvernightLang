import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'astro/config';
import tailwindcss from '@tailwindcss/vite';
import remarkMdLink from './plugins/remark-md-link.mjs';

const tPath = fileURLToPath(new URL('../editors/vscode/syntaxes/evernight.tmLanguage.json', import.meta.url));
const eveGrammar = JSON.parse(readFileSync(tPath, 'utf8'));
eveGrammar.name = 'eve';

const eveTheme = {
  name: 'evernight',
  settings: [
    { settings: { foreground: '#E4E4EF', background: '#15151E', caret: '#C9B8FF' } },
    { scope: ['comment', 'punctuation.definition.comment'], settings: { foreground: '#6B7280', fontStyle: 'italic' } },
    { scope: ['string', 'punctuation.definition.string'], settings: { foreground: '#2FBFA8' } },
    { scope: ['constant.numeric', 'constant.language'], settings: { foreground: '#FBBF24' } },
    { scope: ['keyword', 'storage.type', 'storage.modifier'], settings: { foreground: '#C9B8FF' } },
    { scope: ['entity.name.function', 'support.function', 'variable.function'], settings: { foreground: '#ED93B1' } },
    { scope: ['variable', 'meta.brace'], settings: { foreground: '#E4E4EF' } },
  ],
};

export default defineConfig({
  site: 'https://evernight-lang.org',
  markdown: {
    remarkPlugins: [remarkMdLink],
    shikiConfig: {
      langs: [eveGrammar],
      theme: eveTheme,
    },
  },
  vite: {
    plugins: [tailwindcss()],
  },
});
