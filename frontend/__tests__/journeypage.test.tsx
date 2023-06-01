import React from 'react';
import {describe, it, expect, jest} from '@jest/globals';
import { render, screen, fireEvent } from '@testing-library/react';
import JourneyPage from '../app/journeys/page';

describe('JourneyPage', () => {
  it('renders the pageS buttons correctly', () => {
    render(<JourneyPage />);
    // Assert that the Previous and Next buttons are rendered
    expect(screen.getByText('Previous')).toBeDefined();
    expect(screen.getByText('Next')).toBeDefined;
  });
});
