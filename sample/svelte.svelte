<script>
    import { onMount } from 'svelte';
    let posts = [];

    onMount(async () => {
        const response = await fetch('https://jsonplaceholder.typicode.com/users/1/posts');
        posts = await response.json();
    });
</script>

<main>
    {#each posts as post (post.id)}
        <h2>{post.title}</h2>
        <p>{post.body}</p>
    {/each}
</main>