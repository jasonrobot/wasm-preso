import { TestBed } from '@angular/core/testing';

import { IsPrimeWasmService } from './is-prime-wasm.service';

describe('IsPrimeWasmService', () => {
  let service: IsPrimeWasmService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(IsPrimeWasmService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
