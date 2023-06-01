import {describe, it, expect, test} from '@jest/globals';
import {kilometer, minutes} from "../app/journeys/columns"
import '@testing-library/jest-dom';
 
describe('kilometer', () => {
  it('should convert meters to kilometers', () => {
    expect(kilometer(1000)).toBe(1);
    expect(kilometer(2500)).toBe(2.5);
    expect(kilometer(500)).toBe(0.5);
  });

  it('should round the result to one decimal place', () => {
    expect(kilometer(1500)).toBe(1.5);
    expect(kilometer(3780)).toBe(3.8);
    expect(kilometer(920)).toBe(0.9);
  });

  it('should return 0 when given 0', () => {
    expect(kilometer(0)).toBe(0);
  });
});

describe('minutes', () => {
  it('should return 0 when seconds is less than or equal to 0', () => {
    expect(minutes(0)).toBe(0);
    expect(minutes(-10)).toBe(0);
    expect(minutes(-60)).toBe(0);
  });

  it('should round up when trailing seconds is greater or greater than 30', () => {
    expect(minutes(30)).toBe(1);
    expect(minutes(59)).toBe(1);
    expect(minutes(61)).toBe(1);
    expect(minutes(120)).toBe(2);
    expect(minutes(150)).toBe(3);
  });

  it('should round down when trailing seconds is less than  30', () => {
    expect(minutes(1)).toBe(0);
    expect(minutes(29)).toBe(0);
    expect(minutes(60)).toBe(1);
    expect(minutes(90)).toBe(2);
    expect(minutes(119)).toBe(2);
  });
});