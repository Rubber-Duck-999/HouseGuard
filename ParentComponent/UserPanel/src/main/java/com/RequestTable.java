package com;

import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import com.house_guard.Common.*;

public class RequestTable 
{
	private static Map<Integer,Integer> table;
	private Integer key;
	
	public RequestTable()
	{
		table = new HashMap<Integer, Integer>();
		key = 1;
	}
	
	public void addRecord(Integer pin)
	{
		if(pin < Types.MAXPIN && pin > -1)
		{
			table.put(key, pin); 
			key++;
			//System.out.println("Placing key: " + (key -1) + ", " + pin);
		}
		else
		{
			throw new ArithmeticException("Not a number");
		}
	}
	
	public Integer addRecordNextKeyReturn(Integer pin)
	{
		if(pin < Types.MAXPIN && pin > -1)
		{
			table.put(key, pin); 
			key++;
			//System.out.println("Placing key: " + (key -1) + ", " + pin);
			return (key - 1);
		}
		else
		{
			throw new ArithmeticException("Not a number");
		}
	}
	
	public boolean doesKeyExist(Integer keyValue)
	{
		Iterator<Map.Entry<Integer, Integer>> iterator = 
				table.entrySet().iterator();
		// flag to store result 
		boolean isKeyPresent = false; 

		// Iterate over the HashMap 
		while (iterator.hasNext()) 
		{
			// Get the entry at this iteration 
			Map.Entry<Integer, Integer> entry = iterator.next(); 

			// Check if this key is the required key 
			if (keyValue == entry.getKey()) 
			{ 
				isKeyPresent = true; 
			}	 
		}
		return isKeyPresent; 
	}
	
	public boolean doesValueExist(Integer value)
	{
		Iterator<Map.Entry<Integer, Integer>> iterator = 
				table.entrySet().iterator();
		// flag to store result 
		boolean isValuePresent = false; 

		// Iterate over the HashMap 
		while (iterator.hasNext() && !isValuePresent) 
		{
			// Get the entry at this iteration 
			Map.Entry<Integer, Integer> entry = iterator.next(); 

			// Check if this key is the required key 
			if (value.equals(entry.getValue()))
			{ 
				isValuePresent = true;
			}	 
		}
		return isValuePresent; 
	}
	
	public Integer getValueKey(Integer value)
	{
		Iterator<Map.Entry<Integer, Integer>> iterator = 
				table.entrySet().iterator();
		// flag to store result 
		boolean isValuePresent = false;

		// Iterate over the HashMap 
		while (iterator.hasNext()) 
		{
			// Get the entry at this iteration 
			Map.Entry<Integer, Integer> entry = iterator.next(); 
			// Check if this key is the required key 
			if (value.equals(entry.getValue()))
			{ 
				isValuePresent = true; 
				key = entry.getKey();
			}	 
		}
		return key; 
	}
}
