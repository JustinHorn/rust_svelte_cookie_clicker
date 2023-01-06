<script>
	import logo from '$lib/images/cookie.svg';
	import github from '$lib/images/github.svg';
</script>

<header>
	<div class="corner">
		<img src={logo} alt="Logo" />
	</div>
	<div class="bottom-right">
		<a href="https://github.com/JustinHorn/rust_svelte_cookie_clicker">
			<img src={github} alt="Code" />
		</a>
	</div>
</header>

<style>
	header {
		display: flex;
		justify-content: space-between;
	}

	.corner {
		width: 3em;
		height: 3em;
	}

	

	.corner img {
		width: 2em;
		height: 2em;
		object-fit: contain;
	}

	.bottom-right {
		position: absolute;
		right: 2rem;
		bottom: 2rem;
		width: 3em;
		height: 3em;
	}

</style>