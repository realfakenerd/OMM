<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
    import * as Dialog from "$lib/components/ui/dialog";
</script>

<div class="container mx-auto py-10 space-y-8">
	<h1 class="text-3xl font-bold">shadcn-svelte Demo</h1>

	<section class="space-y-4">
		<h2 class="text-xl font-semibold">Buttons</h2>
		<div class="flex gap-4">
			<Button>Default</Button>
			<Button variant="secondary">Secondary</Button>
			<Button variant="destructive">Destructive</Button>
			<Button variant="outline">Outline</Button>
			<Button variant="ghost">Ghost</Button>
			<Button variant="link">Link</Button>
		</div>
	</section>

	<section class="space-y-4">
		<h2 class="text-xl font-semibold">Card</h2>
		<Card.Root class="w-[350px]">
			<Card.Header>
				<Card.Title>Create project</Card.Title>
				<Card.Description>Deploy your new project in one-click.</Card.Description>
			</Card.Header>
			<Card.Content>
				<form>
					<div class="grid w-full items-center gap-4">
						<div class="flex flex-col space-y-1.5">
							<Label for="name">Name</Label>
							<Input id="name" placeholder="Name of your project" />
						</div>
					</div>
				</form>
			</Card.Content>
			<Card.Footer class="flex justify-between">
				<Button variant="outline">Cancel</Button>
				<Button>Deploy</Button>
			</Card.Footer>
		</Card.Root>
	</section>
    
    <section class="space-y-4">
        <h2 class="text-xl font-semibold">Dialog</h2>
        <Dialog.Root>
            <Dialog.Trigger>
                 {#snippet child({ props })}
                    <Button variant="outline" {...props}>Open Dialog</Button>
                {/snippet}
            </Dialog.Trigger>
            <Dialog.Content>
                <Dialog.Header>
                    <Dialog.Title>Are you sure absolutely sure?</Dialog.Title>
                    <Dialog.Description>
                        This action cannot be undone. This will permanently delete your account and remove your data from our servers.
                    </Dialog.Description>
                </Dialog.Header>
                 <Dialog.Footer>
                     <Button type="submit">Confirm</Button>
                </Dialog.Footer>
            </Dialog.Content>
        </Dialog.Root>
    </section>
</div>
