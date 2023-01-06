<script>
	import Counter from './Counter.svelte';
	import logo from '$lib/images/cookie.svg';
	import { onMount } from 'svelte';

	const basepath = process.env.NODE_ENV === 'production' ? '/' : 'http://localhost:8000/';

	let playerName = "";
	
	let loginName = "";
	let registerName = "";

	let count = 0;

	onMount(
		() => {
			const localPlayerName = localStorage.getItem('playerName');
			if (localPlayerName) {
				playerName = localPlayerName;
				login();
			}
		}
	)


	function handleClick() {
		count += 1;
	}

	function login() {
		fetch(basepath +'player/'+loginName,)
			.then( res => res.json()).then(data => {
				playerName = data.name;
				count = data.count;
				localStorage.setItem('playerName', playerName);
			}).catch(err => console.log(err))
	}

	function register() {
		fetch(basepath + 'player', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({name: registerName})
			}).then( res => res.json()).then(data => {
				playerName = data.name;
				count = data.count;
			}).catch(err => console.log(err))
	}

	function logout() {
		playerName = "";
		count = 0;
		localStorage.removeItem('playerName');
	}

	let mountCount = 0;

	$: count !== mountCount && fetch(basepath + 'player/'+playerName, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({count: count})
	}).catch(err => console.log(err));
</script>

<svelte:head>
	<title>Cookie Clicker</title>
	<meta name="description" content="Cookie Clicker App" />
</svelte:head>

<section>
	<h1>
		Cookie Clicker
	</h1>
	{#if playerName === ""} 
		<div>
			<h3>Login</h3>
			<input type="text" bind:value={loginName} placeholder="Enter your name" />
			<button on:click={() => loginName !== "" && login()}>Submit</button>
		</div>
		<div>
			<h3>Register</h3>
			<input type="text" bind:value={registerName} placeholder="Enter your name" />
			<button on:click={() => registerName !== "" && register()}>Submit</button>
		</div>
	{:else}
		<p>Hello <span class="underscore">{playerName}</span>!</p>
		<Counter bind:count={count}/>
		<span class="welcome">
			<button on:click={handleClick} style="border: none;background: transparent;"> 
				<img src={logo}  alt="cookie-svg" width={count*2+50+"px"} height={count*2+50+"px"}/>
			</button>
		</span>
		<button class="logout" on:click={() => {
			loginName = playerName;
			logout();
		}} >Logout</button>
	{/if}
</section>

<style>

	button {
		cursor: pointer;
	}
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

	.underscore {
		text-decoration: underline;
	}

	.logout {
		position: absolute;
		top: 2rem;
		right: 2rem;
	}
</style>
