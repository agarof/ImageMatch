import { browser } from '$app/env';
import { writable, type Writable } from 'svelte/store';
import type { Token } from './requests';

function read<T>(key: string): T | undefined {
  if (browser && localStorage[key]) {
    return JSON.parse(localStorage[key]);
  }
  return undefined;
}

function write<T>(key: string, value: T | undefined) {
  if (browser) {
    if (value !== undefined) {
      localStorage[key] = JSON.stringify(value);
    } else {
      localStorage.removeItem(key);
    }
  }
}

function create_writable<T>(key: string): Writable<T | undefined> {
  const store = writable(read(key));

  store.subscribe((value) => write(key, value));

  return store;
}

export const token = create_writable<Token>('token');
