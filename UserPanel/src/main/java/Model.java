package com;

public class Model
{
	private final int MAX = 4;
	private int MAXSIZE = 10;
	private int MINSIZE = 0;
	private int[] _digitArray;
	private final Integer PASS = 9000;
	private Integer _attempts;
	
	public Model()
	{
		_attempts = 0;
		_digitArray = new int[4];
	}
	
	public int initModel(int x)
	{
		for(int i = 0; i < 4; i++)
		{
			_digitArray[i] = x;
		}
		return x;
	}

	public int incrementValue(int digit)
	{
		++_digitArray[digit];
		if(_digitArray[digit] >= MAXSIZE)
		{
			_digitArray[digit] = 0;
		}
		System.out.println(_digitArray[digit]);
		return _digitArray[digit];
	}

	public int decrementValue(int digit)
	{
	    --_digitArray[digit];
		if(_digitArray[digit] <= MINSIZE)
		{
			_digitArray[digit] = (MAXSIZE - 1);
		}
		return _digitArray[digit];
	}

	public int checkPass()
	{
		Integer digits = 0;
		digits += _digitArray[0] * 1000;
		digits += _digitArray[1] * 100;
		digits += _digitArray[2] * 10;
		digits += _digitArray[3];
		if(_attempts == Types.MAX_ATTEMPTS)
		{
			return Types.ATTEMPTS_MAXED;
		}
		else if(!digits.equals(PASS))
		{
			return Types.INCORRECT;
		}
		else
		{
			_attempts = 0;
			return Types.CORRECT;
		}
	}
	
	public String setModelStateOn()
	{
		return Types.ON;
	}
	
	public String setModelStateOFF()
	{
		return Types.OFF;
	}
}
