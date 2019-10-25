package com;

public class Model
{
  private int _counter;
  private String _messageReceived;

  public final int getModelValue()
  {
    return _counter;
  }

  public int setModelValue(final int value)
  {
    _counter = value;
    return _counter;
  }

  public Model()
  {
    _messageReceived = "No Error";
  }

  public int incrementValue()
  {
    ++_counter;
    return _counter;
  }

  public void setMessageReceived(String message)
  {
    _messageReceived = message;
  }

  public final String getMessageReceived()
  {
    return _messageReceived;
  }

  public int decrementValue()
  {
    --_counter;
    return _counter;
  }

  public boolean createFile()
  {
    return false;
  }
}
