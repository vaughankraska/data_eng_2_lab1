from pulsar import Function

class SplitterFunc(Function):
  def __init__(self):
    pass

  def process(self, input, context):
    pub_topic = "split";
    if str(input) = "":
      return "\n";
    for word in input.split():
      context.publish(pub_topic, word);
    return;
