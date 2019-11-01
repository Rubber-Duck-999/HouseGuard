package com;

import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;

public class RequestTable 
{
	private static Map<Integer,Integer> table;
	private Integer key;
	
	RequestTable()
	{
		table = new HashMap<Integer, Integer>();
		key = 1;
	}
	
	public void addRecord(Integer pin)
	{
		table.put(key, pin); 
		key++;
		System.out.println("Placing key: " + (key -1) + ", " + pin);
	}
	
	public Integer addRecordReturn(Integer pin)
	{
		table.put(key, pin); 
		key++;
		System.out.println("Placing key: " + (key -1) + ", " + pin);
		return (key - 1);
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
		while (iterator.hasNext()) 
		{
			// Get the entry at this iteration 
			Map.Entry<Integer, Integer> entry = iterator.next(); 

			// Check if this key is the required key 
			if (value == entry.getValue()) 
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
			if (value == entry.getValue()) 
			{ 
				isValuePresent = true; 
				key = entry.getKey();
			}	 
		}
		return key; 
	}
}
