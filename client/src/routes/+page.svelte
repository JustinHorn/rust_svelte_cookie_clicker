<script>
	import Counter from './Counter.svelte';
	import logo from '$lib/images/cookie.svg';

	let count = 0;

	function handleClick() {
		count += 1;
	}
</script>

<svelte:head>
	<title>Cookie Clicker</title>
	<meta name="description" content="Cookie Clicker App" />
</svelte:head>

<section>
		<span class="welcome">
			<button on:click={handleClick} style="border: none;background: transparent;"> 
				<img src={logo}  alt="cookie-svg" width={count*2+50+"px"} height={count*2+50+"px"}/>
			</button>
		</span>

	<h1>
		Cookie Clicker
	</h1>

	<Counter bind:count={count}/>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: flex;
		justify-content: center;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		display: block;
	}
</style>
