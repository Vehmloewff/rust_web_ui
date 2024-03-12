use rust_web_ui::*;

pub struct Example22;

pub struct Example22Props {}

impl Default for Example22Props {
	fn default() -> Example22Props {
		Example22Props { }
	}
}

impl Widget<'_> for Example22 {
	type Props = Example22Props;

	fn render(mut ctx: Ctx<'_>, props: Example22Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("min-h-[768px]"), Style::Color(Color::Fg(11))]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.set_attribute("x-data", "{ open: true }");
				props.set_attribute("@keydown.window.escape", "open = false");
				props.set_attribute("x-init", r#"$watch("open", o => !o && window.setTimeout(() => (open = true), 1000))"#);
				props.set_attribute("x-show", "open");
				props.styles(&[Style::Noop("relative"), Style::Noop("z-10")]);
				props.set_attribute("aria-labelledby", "slide-over-title");
				props.set_attribute("x-ref", "dialog");
				props.set_attribute("aria-modal", "true");
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.set_attribute("x-show", "open");
					props.set_attribute("x-transition:enter", "ease-in-out duration-500");
					props.set_attribute("x-transition:enter-start", "opacity-0");
					props.set_attribute("x-transition:enter-end", "opacity-100");
					props.set_attribute("x-transition:leave", "ease-in-out duration-500");
					props.set_attribute("x-transition:leave-start", "opacity-100");
					props.set_attribute("x-transition:leave-end", "opacity-0");
					props.set_attribute("x-description", "Background backdrop, show/hide based on slide-over state.");
					props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Color(Color::Fg(56)), Style::Noop("bg-opacity-75"), Style::Noop("transition-opacity")]);
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Noop("overflow-hidden")]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("overflow-hidden")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("fixed"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::Noop("max-w-full"), Style::PaddingLeft(Size::Exact(40))]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.set_attribute("x-show", "open");
								props.set_attribute("x-transition:enter", "transform transition ease-in-out duration-500 sm:duration-700");
								props.set_attribute("x-transition:enter-start", "translate-x-full");
								props.set_attribute("x-transition:enter-end", "translate-x-0");
								props.set_attribute("x-transition:leave", "transform transition ease-in-out duration-500 sm:duration-700");
								props.set_attribute("x-transition:leave-start", "translate-x-0");
								props.set_attribute("x-transition:leave-end", "translate-x-full");
								props.styles(&[Style::Noop("pointer-events-auto"), Style::Noop("w-screen"), Style::Noop("max-w-md")]);
								props.set_attribute("x-description", "Slide-over panel, show/hide based on slide-over state.");
								props.set_attribute("@click.away", "open = false");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::Width(Size::Full), Style::Noop("flex-col"), Style::Noop("overflow-y-scroll"), Style::Noop("bg-white"), Style::Noop("shadow-xl")]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("flex-1"), Style::Noop("overflow-y-auto"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))])]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Flex, Style::Noop("items-start"), Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("h2", |mut props| {
												props.styles(&[Style::Noop("text-lg"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												props.set_attribute("id", "slide-over-title");
												
												props.child("0", Label).run(|props| props.text("Shopping cart"));
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Flex, Style::Width(Size::Exact(28)), Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("button", |mut props| {
													props.set_attribute("type", "button");
													props.styles(&[Style::Noop("relative"), Style::Noop("-m-2"), Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::TextColor(Color::Fg(56))])]);
													props.set_attribute("@click", "open = false");
													
													props.child("1", Dynamic).run("span", |mut props| {
														props.styles(&[Style::Noop("absolute"), Style::Noop("-inset-0.5")]);
													});
													props.child("3", Dynamic).run("span", |mut props| {
														props.styles(&[Style::Noop("sr-only")]);
														
														props.child("0", Label).run(|props| props.text("Close panel"));
													});
													props.child("5", Icon).run(|mut props| {
														props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24))]);
													});
												});
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(32))]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("ul", |mut props| {
													props.set_attribute("role", "list");
													props.styles(&[Style::Noop("-my-6"), Style::Noop("divide-y"), Style::Noop("divide-gray-200")]);
													
													props.child("1", Dynamic).run("li", |mut props| {
														props.styles(&[Style::Flex, Style::PaddingY(Size::Exact(24))]);
														
														props.child("1", Dynamic).run("div", |mut props| {
															props.styles(&[Style::Width(Size::Exact(96)), Style::Width(Size::Exact(96)), Style::Noop("flex-shrink-0"), Style::Noop("overflow-hidden"), Style::Noop("rounded-md"), Style::Noop("border"), Style::Noop("border-gray-200")]);
															
															props.child("1", Dynamic).run("img", |mut props| {
																props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/shopping-cart-page-04-product-01.jpg");
																props.set_attribute("alt", "Salmon orange fabric pouch with match zipper, gray zipper pull, and adjustable hip belt.");
																props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
															});
														});
														props.child("3", Dynamic).run("div", |mut props| {
															props.styles(&[Style::MarginLeft(Size::Exact(16)), Style::Flex, Style::Noop("flex-1"), Style::Noop("flex-col")]);
															
															props.child("1", Dynamic).run("div", |mut props| {
																props.child("1", Dynamic).run("div", |mut props| {
																	props.styles(&[Style::Flex, Style::JustifyBetween, Style::Noop("text-base"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																	
																	props.child("1", Dynamic).run("h3", |mut props| {
																		props.child("1", Dynamic).run("a", |mut props| {
																			props.set_attribute("href", "#");
																			
																			props.child("0", Label).run(|props| props.text("Throwback Hip Bag"));
																		});
																	});
																	props.child("3", Dynamic).run("p", |mut props| {
																		props.styles(&[Style::MarginLeft(Size::Exact(16))]);
																		
																		props.child("0", Label).run(|props| props.text("$90.00"));
																	});
																});
																props.child("3", Dynamic).run("p", |mut props| {
																	props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
																	
																	props.child("0", Label).run(|props| props.text("Salmon"));
																});
															});
															props.child("3", Dynamic).run("div", |mut props| {
																props.styles(&[Style::Flex, Style::Noop("flex-1"), Style::Noop("items-end"), Style::JustifyBetween, Style::Noop("text-sm")]);
																
																props.child("1", Dynamic).run("p", |mut props| {
																	props.styles(&[Style::TextColor(Color::Fg(56))]);
																	
																	props.child("0", Label).run(|props| props.text("Qty 1"));
																});
																props.child("3", Dynamic).run("div", |mut props| {
																	props.styles(&[Style::Flex]);
																	
																	props.child("1", Dynamic).run("button", |mut props| {
																		props.set_attribute("type", "button");
																		props.styles(&[Style::Noop("font-medium"), Style::Noop("text-indigo-600"), Style::OnHover(&[Style::Noop("text-indigo-500")])]);
																		
																		props.child("0", Label).run(|props| props.text("Remove"));
																	});
																});
															});
														});
													});
													props.child("3", Dynamic).run("li", |mut props| {
														props.styles(&[Style::Flex, Style::PaddingY(Size::Exact(24))]);
														
														props.child("1", Dynamic).run("div", |mut props| {
															props.styles(&[Style::Width(Size::Exact(96)), Style::Width(Size::Exact(96)), Style::Noop("flex-shrink-0"), Style::Noop("overflow-hidden"), Style::Noop("rounded-md"), Style::Noop("border"), Style::Noop("border-gray-200")]);
															
															props.child("1", Dynamic).run("img", |mut props| {
																props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/shopping-cart-page-04-product-02.jpg");
																props.set_attribute("alt", "Front of satchel with blue canvas body, black straps and handle, drawstring top, and front zipper pouch.");
																props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
															});
														});
														props.child("3", Dynamic).run("div", |mut props| {
															props.styles(&[Style::MarginLeft(Size::Exact(16)), Style::Flex, Style::Noop("flex-1"), Style::Noop("flex-col")]);
															
															props.child("1", Dynamic).run("div", |mut props| {
																props.child("1", Dynamic).run("div", |mut props| {
																	props.styles(&[Style::Flex, Style::JustifyBetween, Style::Noop("text-base"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																	
																	props.child("1", Dynamic).run("h3", |mut props| {
																		props.child("1", Dynamic).run("a", |mut props| {
																			props.set_attribute("href", "#");
																			
																			props.child("0", Label).run(|props| props.text("Medium Stuff Satchel"));
																		});
																	});
																	props.child("3", Dynamic).run("p", |mut props| {
																		props.styles(&[Style::MarginLeft(Size::Exact(16))]);
																		
																		props.child("0", Label).run(|props| props.text("$32.00"));
																	});
																});
																props.child("3", Dynamic).run("p", |mut props| {
																	props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
																	
																	props.child("0", Label).run(|props| props.text("Blue"));
																});
															});
															props.child("3", Dynamic).run("div", |mut props| {
																props.styles(&[Style::Flex, Style::Noop("flex-1"), Style::Noop("items-end"), Style::JustifyBetween, Style::Noop("text-sm")]);
																
																props.child("1", Dynamic).run("p", |mut props| {
																	props.styles(&[Style::TextColor(Color::Fg(56))]);
																	
																	props.child("0", Label).run(|props| props.text("Qty 1"));
																});
																props.child("3", Dynamic).run("div", |mut props| {
																	props.styles(&[Style::Flex]);
																	
																	props.child("1", Dynamic).run("button", |mut props| {
																		props.set_attribute("type", "button");
																		props.styles(&[Style::Noop("font-medium"), Style::Noop("text-indigo-600"), Style::OnHover(&[Style::Noop("text-indigo-500")])]);
																		
																		props.child("0", Label).run(|props| props.text("Remove"));
																	});
																});
															});
														});
													});
													props.child("5", Dynamic).run("li", |mut props| {
														props.styles(&[Style::Flex, Style::PaddingY(Size::Exact(24))]);
														
														props.child("1", Dynamic).run("div", |mut props| {
															props.styles(&[Style::Width(Size::Exact(96)), Style::Width(Size::Exact(96)), Style::Noop("flex-shrink-0"), Style::Noop("overflow-hidden"), Style::Noop("rounded-md"), Style::Noop("border"), Style::Noop("border-gray-200")]);
															
															props.child("1", Dynamic).run("img", |mut props| {
																props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/shopping-cart-page-04-product-03.jpg");
																props.set_attribute("alt", "Front of zip tote bag with white canvas, black canvas straps and handle, and black zipper pulls.");
																props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
															});
														});
														props.child("3", Dynamic).run("div", |mut props| {
															props.styles(&[Style::MarginLeft(Size::Exact(16)), Style::Flex, Style::Noop("flex-1"), Style::Noop("flex-col")]);
															
															props.child("1", Dynamic).run("div", |mut props| {
																props.child("1", Dynamic).run("div", |mut props| {
																	props.styles(&[Style::Flex, Style::JustifyBetween, Style::Noop("text-base"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																	
																	props.child("1", Dynamic).run("h3", |mut props| {
																		props.child("1", Dynamic).run("a", |mut props| {
																			props.set_attribute("href", "#");
																			
																			props.child("0", Label).run(|props| props.text("Zip Tote Basket"));
																		});
																	});
																	props.child("3", Dynamic).run("p", |mut props| {
																		props.styles(&[Style::MarginLeft(Size::Exact(16))]);
																		
																		props.child("0", Label).run(|props| props.text("$140.00"));
																	});
																});
																props.child("3", Dynamic).run("p", |mut props| {
																	props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
																	
																	props.child("0", Label).run(|props| props.text("White and black"));
																});
															});
															props.child("3", Dynamic).run("div", |mut props| {
																props.styles(&[Style::Flex, Style::Noop("flex-1"), Style::Noop("items-end"), Style::JustifyBetween, Style::Noop("text-sm")]);
																
																props.child("1", Dynamic).run("p", |mut props| {
																	props.styles(&[Style::TextColor(Color::Fg(56))]);
																	
																	props.child("0", Label).run(|props| props.text("Qty 1"));
																});
																props.child("3", Dynamic).run("div", |mut props| {
																	props.styles(&[Style::Flex]);
																	
																	props.child("1", Dynamic).run("button", |mut props| {
																		props.set_attribute("type", "button");
																		props.styles(&[Style::Noop("font-medium"), Style::Noop("text-indigo-600"), Style::OnHover(&[Style::Noop("text-indigo-500")])]);
																		
																		props.child("0", Label).run(|props| props.text("Remove"));
																	});
																});
															});
														});
													});
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))])]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Flex, Style::JustifyBetween, Style::Noop("text-base"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("1", Dynamic).run("p", |mut props| {
												props.child("0", Label).run(|props| props.text("Subtotal"));
											});
											props.child("3", Dynamic).run("p", |mut props| {
												props.child("0", Label).run(|props| props.text("$262.00"));
											});
										});
										props.child("3", Dynamic).run("p", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(2)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
											
											props.child("0", Label).run(|props| props.text("Shipping and taxes calculated at checkout."));
										});
										props.child("5", Dynamic).run("div", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(24))]);
											
											props.child("1", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::Noop("border-transparent"), Style::Noop("bg-indigo-600"), Style::PaddingX(Size::Exact(24)), Style::PaddingY(Size::Exact(12)), Style::Noop("text-base"), Style::Noop("font-medium"), Style::Noop("text-white"), Style::Noop("shadow-sm"), Style::OnHover(&[Style::Noop("bg-indigo-700")])]);
												
												props.child("0", Label).run(|props| props.text("Checkout"));
											});
										});
										props.child("7", Dynamic).run("div", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Flex, Style::JustifyCenter, Style::Noop("text-center"), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
											
											props.child("1", Dynamic).run("p", |mut props| {
												props.child("0", Label).run(|props| props.text("or"));
												//  space 
												props.child("3", Dynamic).run("button", |mut props| {
													props.set_attribute("type", "button");
													props.styles(&[Style::Noop("font-medium"), Style::Noop("text-indigo-600"), Style::OnHover(&[Style::Noop("text-indigo-500")])]);
													props.set_attribute("@click", "open = false");
													
													props.child("0", Label).run(|props| props.text("Continue Shopping"));
													props.child("1", Dynamic).run("span", |mut props| {
														props.set_attribute("aria-hidden", "true");
														
														props.child("0", Label).run(|props| props.text("→"));
													});
												});
											});
										});
									});
								});
							});
						});
					});
				});
			});
		});
	}
}
