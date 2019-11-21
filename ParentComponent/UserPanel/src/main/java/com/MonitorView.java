package com;

import java.awt.BorderLayout;
import com.house_guard.Common.*;
import java.awt.Color;
import java.awt.FlowLayout;
import java.awt.Font;
import java.awt.GridBagConstraints;
import java.awt.GridBagLayout;
import java.awt.GridLayout;
import java.awt.event.*;

import javax.swing.JButton;
import javax.swing.JFrame;
import javax.swing.JLabel;
import javax.swing.JOptionPane;
import javax.swing.JPanel;
import javax.swing.JTextField;
import javax.swing.SwingConstants;

import java.util.logging.Logger;

public class MonitorView
{
	private JFrame frame;
	private JPanel buttonPanel, fieldPanel;
	private JLabel monitorState;
	private JButton ONButton, OFFButton;
	private Font buttonFont;

	public MonitorView()
	{
		frame = new JFrame("Simple MVC");
		buttonPanel = new JPanel();
		fieldPanel = new JPanel();

		monitorState = new JLabel("", SwingConstants.CENTER);

		ONButton = new JButton("ON");
		OFFButton = new JButton("OFF");

		buttonPanel.add(ONButton);
		buttonPanel.add(OFFButton);

		buttonPanel.setLayout(new GridLayout(2,4));

		setButtonFont();
		setLabelFont();

		fieldPanel.setLayout(new GridLayout(1,2));
		fieldPanel.add(monitorState);

		frame.add(fieldPanel, BorderLayout.NORTH);
		frame.add(buttonPanel, BorderLayout.CENTER);
	}

	public void setMonitor()
	{
		frame.setDefaultCloseOperation(JFrame.HIDE_ON_CLOSE);
		frame.setExtendedState(JFrame.MAXIMIZED_BOTH);
		//frame.setUndecorated(true);
		frame.setSize(640,480);
		//frame.pack();
		frame.setVisible(true);
	}

	private void setLabelFont()
	{
		buttonFont = new Font("Calibri", Font.BOLD, 128);
		monitorState.setFont(buttonFont);
	}

	private void setButtonFont()
	{
		buttonFont = new Font("Calibri", Font.BOLD, 40);
		ONButton.setFont(buttonFont);
		ONButton.setForeground(Color.GREEN);
		OFFButton.setFont(buttonFont);
		OFFButton.setForeground(Color.GREEN);
	}

	public void close()
	{
		frame.dispose();
	}

	public < E > void setMonitorState(E v)
	{
		monitorState.setText("" + v);
	}

	public void addController(ActionListener listenerButtons)
	{
		ONButton.setActionCommand(Types.State.ON.name());
		OFFButton.setActionCommand(Types.State.OFF.name());
		ONButton.addActionListener(listenerButtons);
		OFFButton.addActionListener(listenerButtons);
	}

	public static class CloseListener extends WindowAdapter
	{
		public void windowClosing(WindowEvent e)
		{
			e.getWindow().setVisible(false);
			System.exit(0);
		}
	}
}
