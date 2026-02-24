<script lang="ts">
  import { setContext } from 'svelte';
  import type { Component } from 'svelte';

  interface StackEntry {
    component: Component<any>;
    props: Record<string, any>;
    id: string;
  }

  export interface StackContext {
    push: (component: Component<any>, props?: Record<string, any>) => void;
    pop: () => void;
    replace: (component: Component<any>, props?: Record<string, any>) => void;
    canPop: () => boolean;
  }

  interface Props {
    initialComponent: Component<any>;
    initialProps?: Record<string, any>;
    contextKey?: string;
  }

  let { initialComponent, initialProps = {}, contextKey = 'stack' }: Props = $props();

  let stack = $state<StackEntry[]>([
    { component: initialComponent, props: initialProps, id: crypto.randomUUID() }
  ]);

  function push(component: Component<any>, props: Record<string, any> = {}) {
    stack.push({ component, props, id: crypto.randomUUID() });
  }

  function pop() {
    if (stack.length > 1) stack.pop();
  }

  function replace(component: Component<any>, props: Record<string, any> = {}) {
    stack[stack.length - 1] = { component, props, id: crypto.randomUUID() };
  }

  function canPop() {
    return stack.length > 1;
  }

  setContext<StackContext>(contextKey, { push, pop, replace, canPop });
</script>

{#each stack as page, i (page.id)}
  <div style:display={i === stack.length - 1 ? 'contents' : 'none'} style="width: 100%; height: 100%">
    <page.component {...page.props} />
  </div>
{/each}