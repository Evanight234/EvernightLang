import { defineCollection, z } from 'astro:content';
import { glob } from 'astro/loaders';

const docs = defineCollection({
  loader: glob({
    base: '../',
    pattern: [
      'docs/**/*.md',
      'KEYWORD.md',
      'TIPE.md',
      'FUNCTION.md',
      'ERROR.md',
      'STDLIB.md',
      'GRAMMAR.md',
      'ARRAY.md',
    ],
  }),
  schema: z.object({
    judul: z.string().optional(),
    grup: z.string().optional(),
    urutan: z.number().optional(),
  }),
});

export const collections = { docs };
