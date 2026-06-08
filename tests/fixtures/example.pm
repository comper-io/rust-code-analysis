#!/usr/bin/env perl
use strict;
use warnings;

package Example;

sub new {
    my ($class, %args) = @_;
    return bless \%args, $class;
}

sub greet {
    my ($self, $name) = @_;
    return "Hello, $name!" if $name;
    return "Hello, world!";
}

sub count_to {
    my ($self, $n) = @_;
    my $total = 0;
    for my $i (1 .. $n) {
        $total += $i;
    }
    return $total;
}

1;
