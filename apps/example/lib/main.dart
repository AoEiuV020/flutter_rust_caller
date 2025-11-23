import 'package:flutter/material.dart';
import 'package:flutter_rust_caller/flutter_rust_caller.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Prime Number Calculator',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
      ),
      home: const MyHomePage(title: 'Prime Number Calculator'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _currentNumber = 2;
  bool _isLoading = false;

  /// 乘以 10（两参数）
  void _multiplyBy10() async {
    setState(() => _isLoading = true);
    try {
      final result = await rustCallAsync('Multiply', {
        'base': _currentNumber,
        'multiplier': 10,
      });
      setState(() {
        _currentNumber = result['result'] ?? 0;
      });
    } catch (e) {
      print('Error: $e');
    } finally {
      setState(() => _isLoading = false);
    }
  }

  /// 乘以 10 的 10 次方（三参数：base * (exponent_base ^ exponent)）
  void _multiplyBy10Power() async {
    setState(() => _isLoading = true);
    try {
      final result = await rustCallAsync('PowerMultiply', {
        'base': _currentNumber,
        'exponent_base': 10,
        'exponent': 10,
      });
      setState(() {
        _currentNumber = result['result'] ?? 0;
      });
    } catch (e) {
      print('Error: $e');
    } finally {
      setState(() => _isLoading = false);
    }
  }

  /// 找下 100 个质数（两参数）
  void _find100Primes() async {
    setState(() => _isLoading = true);
    try {
      final result = await rustCallAsync('NextPrime', {
        'base': _currentNumber,
        'count': 100,
      });
      setState(() {
        _currentNumber = result['result'] ?? 0;
      });
    } catch (e) {
      print('Error: $e');
    } finally {
      setState(() => _isLoading = false);
    }
  }

  /// 获取静态保存的质数结果
  void _getLastPrime() async {
    setState(() => _isLoading = true);
    try {
      final result = await rustCallAsync('GetLastPrime', {});
      setState(() {
        _currentNumber = result['result'] ?? 0;
      });
    } catch (e) {
      print('Error: $e');
    } finally {
      setState(() => _isLoading = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text('Current Number:'),
            Text(
              '$_currentNumber',
              style: Theme.of(context).textTheme.headlineMedium,
            ),
            const SizedBox(height: 20),
            const CircularProgressIndicator(),
          ],
        ),
      ),
      floatingActionButton: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          FloatingActionButton(
            onPressed: _isLoading ? null : _multiplyBy10,
            tooltip: 'Multiply by 10',
            child: const Icon(Icons.close),
          ),
          const SizedBox(width: 16),
          FloatingActionButton(
            onPressed: _isLoading ? null : _multiplyBy10Power,
            tooltip: 'Multiply by 10^10',
            child: const Icon(Icons.power),
          ),
          const SizedBox(width: 16),
          FloatingActionButton(
            onPressed: _isLoading ? null : _find100Primes,
            tooltip: 'Find 100 Primes',
            child: const Icon(Icons.arrow_forward),
          ),
          const SizedBox(width: 16),
          FloatingActionButton(
            onPressed: _isLoading ? null : _getLastPrime,
            tooltip: 'Get Last Prime',
            child: const Icon(Icons.save),
          ),
        ],
      ),
    );
  }
}
