-- phpMyAdmin SQL Dump
-- version 5.2.0
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1
-- Generation Time: Mar 29, 2023 at 05:15 PM
-- Server version: 10.4.24-MariaDB
-- PHP Version: 8.1.6

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `lottery`
--

-- --------------------------------------------------------

--
-- Table structure for table `admin`
--

CREATE TABLE `admin` (
  `admin_id` int(64) NOT NULL,
  `F_Name` varchar(32) COLLATE utf8_unicode_ci NOT NULL,
  `L_Name` varchar(32) COLLATE utf8_unicode_ci NOT NULL,
  `Username` varchar(16) COLLATE utf8_unicode_ci NOT NULL,
  `Password` varchar(16) COLLATE utf8_unicode_ci NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

--
-- Dumping data for table `admin`
--

INSERT INTO `admin` (`admin_id`, `F_Name`, `L_Name`, `Username`, `Password`) VALUES
(1, 'Putipong', 'Rungthaweesup', 'admin', 'admin123');

-- --------------------------------------------------------

--
-- Table structure for table `basket`
--

CREATE TABLE `basket` (
  `user_id` int(64) NOT NULL,
  `lottery_id` int(100) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

--
-- Dumping data for table `basket`
--

INSERT INTO `basket` (`user_id`, `lottery_id`) VALUES
(1, 1),
(1, 2);

-- --------------------------------------------------------

--
-- Table structure for table `lottery`
--

CREATE TABLE `lottery` (
  `lottery_id` int(100) NOT NULL,
  `lottery_number` varchar(6) COLLATE utf8_unicode_ci NOT NULL,
  `lottery_status` varchar(9) COLLATE utf8_unicode_ci NOT NULL,
  `Datetime` datetime NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

--
-- Dumping data for table `lottery`
--

INSERT INTO `lottery` (`lottery_id`, `lottery_number`, `lottery_status`, `Datetime`) VALUES
(1, '123456', 'available', '2023-03-27 15:55:23'),
(2, '234567', 'available', '2023-03-27 15:57:14'),
(3, '345678', 'sold-out', '2023-03-27 15:59:47'),
(4, '456789', 'sold-out', '2023-03-27 16:00:16');

-- --------------------------------------------------------

--
-- Table structure for table `reward`
--

CREATE TABLE `reward` (
  `reward_id` int(32) NOT NULL,
  `reward_number` varchar(6) COLLATE utf8_unicode_ci NOT NULL,
  `Datetime` datetime NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

--
-- Dumping data for table `reward`
--

INSERT INTO `reward` (`reward_id`, `reward_number`, `Datetime`) VALUES
(1, '987654', '2023-03-27 16:11:18');

-- --------------------------------------------------------

--
-- Table structure for table `user`
--

CREATE TABLE `user` (
  `user_id` int(64) NOT NULL,
  `F_Name` varchar(32) COLLATE utf8_unicode_ci NOT NULL,
  `L_Name` varchar(32) COLLATE utf8_unicode_ci NOT NULL,
  `Username` varchar(16) COLLATE utf8_unicode_ci NOT NULL,
  `Password` varchar(16) COLLATE utf8_unicode_ci NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

--
-- Dumping data for table `user`
--

INSERT INTO `user` (`user_id`, `F_Name`, `L_Name`, `Username`, `Password`) VALUES
(1, 'humnoi', 'noinoi', 'humnoi', 'noi'),
(2, 'humyai', 'yaiyai', 'humyai', 'yai');

-- --------------------------------------------------------

--
-- Table structure for table `user_history`
--

CREATE TABLE `user_history` (
  `user_id` int(64) NOT NULL,
  `lottery_number` varchar(6) COLLATE utf8_unicode_ci NOT NULL,
  `Datetime` datetime NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

--
-- Dumping data for table `user_history`
--

INSERT INTO `user_history` (`user_id`, `lottery_number`, `Datetime`) VALUES
(2, '345678', '2023-03-27 16:00:56'),
(2, '456789', '2023-03-27 16:01:21');

--
-- Indexes for dumped tables
--

--
-- Indexes for table `admin`
--
ALTER TABLE `admin`
  ADD PRIMARY KEY (`admin_id`);

--
-- Indexes for table `lottery`
--
ALTER TABLE `lottery`
  ADD PRIMARY KEY (`lottery_id`);

--
-- Indexes for table `reward`
--
ALTER TABLE `reward`
  ADD PRIMARY KEY (`reward_id`);

--
-- Indexes for table `user`
--
ALTER TABLE `user`
  ADD PRIMARY KEY (`user_id`);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table `admin`
--
ALTER TABLE `admin`
  MODIFY `admin_id` int(64) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=2;

--
-- AUTO_INCREMENT for table `lottery`
--
ALTER TABLE `lottery`
  MODIFY `lottery_id` int(100) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=5;

--
-- AUTO_INCREMENT for table `reward`
--
ALTER TABLE `reward`
  MODIFY `reward_id` int(32) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=2;

--
-- AUTO_INCREMENT for table `user`
--
ALTER TABLE `user`
  MODIFY `user_id` int(64) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=3;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
