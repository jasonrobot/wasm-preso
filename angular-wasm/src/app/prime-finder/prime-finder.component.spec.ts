import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PrimeFinderComponent } from './prime-finder.component';

describe('PrimeFinderComponent', () => {
  let component: PrimeFinderComponent;
  let fixture: ComponentFixture<PrimeFinderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PrimeFinderComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(PrimeFinderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
