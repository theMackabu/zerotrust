import { globSync } from 'glob';
import { writeFileSync, readFileSync } from 'fs';
import type { AstroIntegration, AstroConfig } from 'astro';

const replaceText = (input): AstroIntegration => {
	return {
		name: 'replace',
		hooks: {
			'astro:build:done': async ({ dir }) => {
				globSync(`${decodeURI(dir.pathname)}**/*.html`).forEach((filePath) => {
					let html = readFileSync(filePath, 'utf8');

					for (let key in input) {
						if (input.hasOwnProperty(key)) {
							html = html.replace(new RegExp(key, 'g'), input[key]);
						}
					}

					writeFileSync(filePath, html, 'utf8');
				});
			}
		}
	};
};

export default function (input): AstroIntegration {
	return {
		name: 'replace',
		hooks: {
			'astro:config:setup': ({ _, updateConfig }) => {
				updateConfig({ integrations: [replaceText(input)] });
			}
		}
	};
}
