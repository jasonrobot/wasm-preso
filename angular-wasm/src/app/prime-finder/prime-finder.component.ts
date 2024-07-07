import { Component } from '@angular/core';
import { IsPrimeWasmService } from '../is-prime-wasm.service';
import { AsyncPipe } from '@angular/common';

@Component({
  selector: 'app-prime-finder',
  standalone: true,
  imports: [AsyncPipe],
  templateUrl: './prime-finder.component.html',
  styleUrl: './prime-finder.component.scss'
})
export class PrimeFinderComponent {
  constructor(
    public isPrimeService: IsPrimeWasmService
  ) {}

  public value: number = Math.floor(Math.random() * 100);

  findPrime() {
    const result = this.isPrimeService.isPrime(this.value);
    return result
  }
}
