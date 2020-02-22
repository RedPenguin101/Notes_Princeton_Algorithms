# Searches and Sorts

## Big O Summary

* O(n2): Bubble Sort, Selection Sort, Insertion Sort
* O(n log n): Merge sort
* O(n): Linear search
* O(log n): Binary search
* O(1)

## Search

### Linear Search

In pseudocode:

  if first element is what you're looking for
    stop
  else move to the next element and repeat

### Binary Search

Requires a sorted collection

In pseudocode:

  look at the element at the midpoint
  If the result is th element you're looking for
    stop
  else if the result is larger than you're looking for
    search the left half of the collection
  else if the results is smaller than you're looking for
    search the right half of the collection

## Sorts

### Bubble Sort

In pseudocode

  given a collection of comparable elements

  iterate through the collection, looking at each element
    if the element is larger than the the element on it's right, swap them

  repeat until sorted.

### Selection Sort

In pseudocode

  given a collection of comparable elements

  starting from n loop through the collection, finding the smallest element
  swap the resulting element with position n
  repeat, but starting at position n+1

### Insertion Sort


### Merge Sort

In psuedocode

  if collection size is 1
    return the element
  else
    sort the left half
    sort the right half
    merge the halves
