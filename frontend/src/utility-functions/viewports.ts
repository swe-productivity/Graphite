import { type Editor } from "@graphite/editor";

export function updateBoundsOfViewports(editor: Editor) {
	const viewports = Array.from(window.document.querySelectorAll("[data-viewport-container]"));

	if (viewports.length <= 0) return;

	const viewportBounds = viewports[0].getBoundingClientRect();

	const left = viewportBounds.left;
	const top = viewportBounds.top;
	const right = viewportBounds.right;
	const bottom = viewportBounds.bottom;

	const scale = window.devicePixelRatio || 1;

	editor.handle.updateViewport(left, top, right, bottom, scale);
}
