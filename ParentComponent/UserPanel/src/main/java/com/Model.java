package com;

import com.house_guard.Common.*;

public class Model
{
	private final int MAX = 4;
	private int MAXSIZE = 10;
	private int MINSIZE = 0;
	private int[] _digitArray;
	
	public Model()
	{
		_digitArray = new int[4];
		for(int i = 0; i < 4; i++)
		{
			_digitArray[i] = 0;
		}
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
		_digitArray[digit] = _digitArray[digit] + 1;
		if(_digitArray[digit] >= MAXSIZE)
		{
			_digitArray[digit] = 0;
		}
		return _digitArray[digit];
	}

	public int decrementValue(int digit)
	{
	    --_digitArray[digit];
		if(_digitArray[digit] < MINSIZE)
		{
			_digitArray[digit] = (MAXSIZE - 1);
		}
		return _digitArray[digit];
	}

	public Integer checkPass()
	{
		Integer digits = 0;
		digits += _digitArray[0] * 1000;
		digits += _digitArray[1] * 100;
		digits += _digitArray[2] * 10;
		digits += _digitArray[3];
		return digits;
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
