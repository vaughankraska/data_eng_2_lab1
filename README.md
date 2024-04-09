# Data Engineering II Lab I
## My implementation
- Use rust to implement the producers and consumers
- Use the gutenberg project disribution of the pride and prejudice book as the text we are are performing our capization process on
## Assignment Description:
```text
The repository consists of a conversion.py file which demonstrates a code that has a
“conversion” operation applied to each word on a string. Assume that there is a requirement that
the operation is applied on each word instead of the entire string. Based on this premise, you
are required to complete the following sub-tasks:
1. Identify an issue with the current implementation in terms of handling big data i.e., words
are in the order of millions.
2. Redesign the current implementation by using apache pulsar to demonstrate splitting
and merging of data i.e., how the same operation on each word (i.e., splitting) and the
resultant string (i.e., merging) can be handled by consumers and producers. Provide a
diagram to demonstrate how your architecture will look like i.e.,
a. How data splitting and merging is handled by consumer and producer
b. Label broker, consumer and producer.
3. Provide an implementation (preferably in Python programming language) of your
architecture with Apache Pulsar. You can set up your architecture on a single virtual
machine by creating different sessions for each consumer and producer.
```
* The file:
```python
#!/usr/bin/env python3
import time

"""
    The implementation is used to demonstrate an intensive "conversion" function on elements
"""

# Fill in your author information
___author___ = ""
___email____ = ""

# Input string
INPUT_STRING = "I want to be capatilized"

# Iteration represents the operation applied to each word in the string 
# e.g., 1 represents that operation is applied to first word in the string
ITERATION = 5

def conversion(substring, operation):
    """A conversion function which takes a string as an input and outputs a converted string

    Args:
        substring (String)
        operation (function): This is an operation on the given input

    Returns:
        [String]: Converted String
    """


    # returns the conversion applied to input
    return function(substring)



def function(string):
    """ A function that performs some operation on a string. You can change the operation accordingly

    Args:
        string (String): input string on which some operation is applied

    Returns:
        [String]: string in upper case
    """
    return string.upper()





if __name__ == "__main__":

    # Check for correct input
    if ITERATION > len(INPUT_STRING.split()):
        print ("Iteration cannot be greater than the number of words in a string")
        print ("Terminating the benchmark")
        exit()

    print ("Original String: {}".format(INPUT_STRING))
    resultant_string = ""
    


    split_string = INPUT_STRING.split(" ")


    for i in range(0,ITERATION):

        upper_case_string = conversion(split_string[i], function)
        
        resultant_string +=   upper_case_string  + ' '    


    print ("Resultant String: {}".format(resultant_string))
```
