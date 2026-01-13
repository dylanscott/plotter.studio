<script lang="ts">
	// Placeholder component for PixiJS canvas - to be implemented later
</script>

<div class="canvas-container">
	<span class="placeholder-text">PREVIEW CANVAS</span>
</div>

<style>
	.canvas-container {
		width: 100%;
		height: 100%;
		background-color: var(--bg-primary);
		background-image:
			linear-gradient(var(--grid-line) 1px, transparent 1px),
			linear-gradient(90deg, var(--grid-line) 1px, transparent 1px);
		background-size: 20px 20px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.placeholder-text {
		font-family: var(--main-font);
		text-transform: uppercase;
		letter-spacing: 0.2em;
		color: var(--text-tertiary);
		font-size: 0.875rem;
	}
</style>
