import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable, filter, map } from 'rxjs';

import IsPrimeModule from '../assets/wasm/is-prime.js';
// import '../assets/wasm/is-prime.wasm'

type is_prime = (n: number) => 1 | 0;

@Injectable({
  providedIn: 'root'
})
export class IsPrimeWasmService {

  // public wasm: {is_prime: is_prime} | null = null;

  private wasm: any;

  constructor() {
    this.loadWasm('../assets/wasm/is-prime.wasm');
  }

  public loaded = new BehaviorSubject<boolean>(false);

  async loadWasm(path: string) {
    const wasmFile = await fetch(path);
    const wasmBinary = new Uint8Array(await wasmFile.arrayBuffer());

    this.wasm = await IsPrimeModule({
      onRuntimeInitialized: () => this.loaded.next(true),
      wasmBinary,
    })
  }

  public isPrime(n: number): Observable<boolean> {
    return this.loaded.pipe(
      filter(isLoaded => isLoaded && this.wasm),
      map(() => this.wasm._is_prime(n)),
      map(result => Boolean(result)),
    )
  }
}
