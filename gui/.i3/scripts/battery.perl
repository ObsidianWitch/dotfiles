#!/usr/bin/perl
#
# Copyright 2014 Pierre Mavro <deimos@deimos.fr>
# Copyright 2014 Vivien Didelot <vivien@didelot.org>
#
# Licensed under the terms of the GNU GPL v3, or any later version.
#
# This script is meant to use with i3blocks. It parses the output of the "acpi"
# command (often provided by a package of the same name) to read the status of
# the battery, and eventually its remaining time (to full charge or discharge).

use strict;
use warnings;
use utf8;

my $acpi;
my $status;
my $percent;
my $full_text;
my $short_text;

# read the first line of the "acpi" command output
open (ACPI, 'acpi -b |') or die "Can't exec acpi: $!\n";
$acpi = <ACPI>;
close(ACPI);

# fail on unexpected output
if ($acpi !~ /: (\w+), (\d+)%/) {
	die "$acpi\n";
}

$status = $1;
$percent = $2;
$full_text = "$percent%";

if ($status eq 'Discharging') {
	$full_text .= ' DIS';
} elsif ($status eq 'Charging') {
	$full_text .= ' CHR';
}

$short_text = $full_text;

if ($acpi =~ /(\d\d:\d\d):/) {
	$full_text .= " ($1)";
}

# print text
print "$full_text\n";

exit(0);
