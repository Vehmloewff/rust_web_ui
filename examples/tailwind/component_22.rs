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
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("min-h-[768px]"), Style::Color(Color::Fg(11))]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.set_attribute("x-data", "{ open: true }");
				props.set_attribute("@keydown.window.escape", "open = false");
				props.set_attribute("x-init", r#"$watch("open", o => !o && window.setTimeout(() => (open = true), 1000))"#);
				props.set_attribute("x-show", "open");
				props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("z-10")]);
				props.set_attribute("aria-labelledby", "slide-over-title");
				props.set_attribute("x-ref", "dialog");
				props.set_attribute("aria-modal", "true");
				
				props.child("1", Dynamic).run("div", |props| {
					props.set_attribute("x-show", "open");
					props.set_attribute("x-transition:enter", "ease-in-out duration-500");
					props.set_attribute("x-transition:enter-start", "opacity-0");
					props.set_attribute("x-transition:enter-end", "opacity-100");
					props.set_attribute("x-transition:leave", "ease-in-out duration-500");
					props.set_attribute("x-transition:leave-start", "opacity-100");
					props.set_attribute("x-transition:leave-end", "opacity-0");
					props.set_attribute("x-description", "Background backdrop, show/hide based on slide-over state.");
					props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), Style::Color(Color::Fg(56)), NoStyle::Noop("bg-opacity-75"), NoStyle::Noop("transition-opacity")]);
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), NoStyle::Noop("overflow-hidden")]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("overflow-hidden")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("fixed"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, NoStyle::Noop("max-w-full"), Style::PaddingLeft(40)]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.set_attribute("x-show", "open");
								props.set_attribute("x-transition:enter", "transform transition ease-in-out duration-500 sm:duration-700");
								props.set_attribute("x-transition:enter-start", "translate-x-full");
								props.set_attribute("x-transition:enter-end", "translate-x-0");
								props.set_attribute("x-transition:leave", "transform transition ease-in-out duration-500 sm:duration-700");
								props.set_attribute("x-transition:leave-start", "translate-x-0");
								props.set_attribute("x-transition:leave-end", "translate-x-full");
								props.styles(&[NoStyle::Noop("pointer-events-auto"), NoStyle::Noop("w-screen"), NoStyle::Noop("max-w-md")]);
								props.set_attribute("x-description", "Slide-over panel, show/hide based on slide-over state.");
								props.set_attribute("@click.away", "open = false");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, NoStyle::Noop("h-full"), NoStyle::Noop("flex-col"), NoStyle::Noop("overflow-y-scroll"), NoStyle::Noop("bg-white"), NoStyle::Noop("shadow-xl")]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("flex-1"), NoStyle::Noop("overflow-y-auto"), Style::PaddingX(16), Style::PaddingY(24), Screen::Small(&[Style::PaddingX(24)])]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::Flex, NoStyle::Noop("items-start"), Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("h2", |props| {
												props.styles(&[NoStyle::Noop("text-lg"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												props.set_attribute("id", "slide-over-title");
												
												props.child("0", Label).run(|props| props.set_text("Shopping cart"));
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::MarginLeft(12), Style::Flex, Style::Width(28), Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("button", |props| {
													props.set_attribute("type", "button");
													props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("-m-2"), Style::Padding(8), Style::TextColor(Color::Fg(44)), Action::Hover(&[Style::TextColor(Color::Fg(56))])]);
													props.set_attribute("@click", "open = false");
													
													props.child("1", Dynamic).run("span", |props| {
														props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("-inset-0.5")]);
													});
													props.child("3", Dynamic).run("span", |props| {
														props.styles(&[NoStyle::Noop("sr-only")]);
														
														props.child("0", Label).run(|props| props.set_text("Close panel"));
													});
													props.child("5", Icon).run(|props| {
														props.style(&[Style::Width(24), Style::Width(24)]);
													});
												});
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(32)]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("ul", |props| {
													props.set_attribute("role", "list");
													props.styles(&[NoStyle::Noop("-my-6"), NoStyle::Noop("divide-y"), NoStyle::Noop("divide-gray-200")]);
													
													props.child("1", Dynamic).run("li", |props| {
														props.styles(&[Style::Flex, Style::PaddingY(24)]);
														
														props.child("1", Dynamic).run("div", |props| {
															props.styles(&[Style::Width(96), Style::Width(96), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), NoStyle::Noop("border-gray-200")]);
															
															props.child("1", Dynamic).run("img", |props| {
																props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/shopping-cart-page-04-product-01.jpg");
																props.set_attribute("alt", "Salmon orange fabric pouch with match zipper, gray zipper pull, and adjustable hip belt.");
																props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
															});
														});
														props.child("3", Dynamic).run("div", |props| {
															props.styles(&[Style::MarginLeft(16), Style::Flex, NoStyle::Noop("flex-1"), NoStyle::Noop("flex-col")]);
															
															props.child("1", Dynamic).run("div", |props| {
																props.child("1", Dynamic).run("div", |props| {
																	props.styles(&[Style::Flex, Style::JustifyBetween, NoStyle::Noop("text-base"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																	
																	props.child("1", Dynamic).run("h3", |props| {
																		props.child("1", Dynamic).run("a", |props| {
																			props.set_attribute("href", "#");
																			
																			props.child("0", Label).run(|props| props.set_text("Throwback Hip Bag"));
																		});
																	});
																	props.child("3", Dynamic).run("p", |props| {
																		props.styles(&[Style::MarginLeft(16)]);
																		
																		props.child("0", Label).run(|props| props.set_text("$90.00"));
																	});
																});
																props.child("3", Dynamic).run("p", |props| {
																	props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
																	
																	props.child("0", Label).run(|props| props.set_text("Salmon"));
																});
															});
															props.child("3", Dynamic).run("div", |props| {
																props.styles(&[Style::Flex, NoStyle::Noop("flex-1"), NoStyle::Noop("items-end"), Style::JustifyBetween, NoStyle::Noop("text-sm")]);
																
																props.child("1", Dynamic).run("p", |props| {
																	props.styles(&[Style::TextColor(Color::Fg(56))]);
																	
																	props.child("0", Label).run(|props| props.set_text("Qty 1"));
																});
																props.child("3", Dynamic).run("div", |props| {
																	props.styles(&[Style::Flex]);
																	
																	props.child("1", Dynamic).run("button", |props| {
																		props.set_attribute("type", "button");
																		props.styles(&[NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("text-indigo-500")])]);
																		
																		props.child("0", Label).run(|props| props.set_text("Remove"));
																	});
																});
															});
														});
													});
													props.child("3", Dynamic).run("li", |props| {
														props.styles(&[Style::Flex, Style::PaddingY(24)]);
														
														props.child("1", Dynamic).run("div", |props| {
															props.styles(&[Style::Width(96), Style::Width(96), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), NoStyle::Noop("border-gray-200")]);
															
															props.child("1", Dynamic).run("img", |props| {
																props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/shopping-cart-page-04-product-02.jpg");
																props.set_attribute("alt", "Front of satchel with blue canvas body, black straps and handle, drawstring top, and front zipper pouch.");
																props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
															});
														});
														props.child("3", Dynamic).run("div", |props| {
															props.styles(&[Style::MarginLeft(16), Style::Flex, NoStyle::Noop("flex-1"), NoStyle::Noop("flex-col")]);
															
															props.child("1", Dynamic).run("div", |props| {
																props.child("1", Dynamic).run("div", |props| {
																	props.styles(&[Style::Flex, Style::JustifyBetween, NoStyle::Noop("text-base"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																	
																	props.child("1", Dynamic).run("h3", |props| {
																		props.child("1", Dynamic).run("a", |props| {
																			props.set_attribute("href", "#");
																			
																			props.child("0", Label).run(|props| props.set_text("Medium Stuff Satchel"));
																		});
																	});
																	props.child("3", Dynamic).run("p", |props| {
																		props.styles(&[Style::MarginLeft(16)]);
																		
																		props.child("0", Label).run(|props| props.set_text("$32.00"));
																	});
																});
																props.child("3", Dynamic).run("p", |props| {
																	props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
																	
																	props.child("0", Label).run(|props| props.set_text("Blue"));
																});
															});
															props.child("3", Dynamic).run("div", |props| {
																props.styles(&[Style::Flex, NoStyle::Noop("flex-1"), NoStyle::Noop("items-end"), Style::JustifyBetween, NoStyle::Noop("text-sm")]);
																
																props.child("1", Dynamic).run("p", |props| {
																	props.styles(&[Style::TextColor(Color::Fg(56))]);
																	
																	props.child("0", Label).run(|props| props.set_text("Qty 1"));
																});
																props.child("3", Dynamic).run("div", |props| {
																	props.styles(&[Style::Flex]);
																	
																	props.child("1", Dynamic).run("button", |props| {
																		props.set_attribute("type", "button");
																		props.styles(&[NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("text-indigo-500")])]);
																		
																		props.child("0", Label).run(|props| props.set_text("Remove"));
																	});
																});
															});
														});
													});
													props.child("5", Dynamic).run("li", |props| {
														props.styles(&[Style::Flex, Style::PaddingY(24)]);
														
														props.child("1", Dynamic).run("div", |props| {
															props.styles(&[Style::Width(96), Style::Width(96), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), NoStyle::Noop("border-gray-200")]);
															
															props.child("1", Dynamic).run("img", |props| {
																props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/shopping-cart-page-04-product-03.jpg");
																props.set_attribute("alt", "Front of zip tote bag with white canvas, black canvas straps and handle, and black zipper pulls.");
																props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
															});
														});
														props.child("3", Dynamic).run("div", |props| {
															props.styles(&[Style::MarginLeft(16), Style::Flex, NoStyle::Noop("flex-1"), NoStyle::Noop("flex-col")]);
															
															props.child("1", Dynamic).run("div", |props| {
																props.child("1", Dynamic).run("div", |props| {
																	props.styles(&[Style::Flex, Style::JustifyBetween, NoStyle::Noop("text-base"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																	
																	props.child("1", Dynamic).run("h3", |props| {
																		props.child("1", Dynamic).run("a", |props| {
																			props.set_attribute("href", "#");
																			
																			props.child("0", Label).run(|props| props.set_text("Zip Tote Basket"));
																		});
																	});
																	props.child("3", Dynamic).run("p", |props| {
																		props.styles(&[Style::MarginLeft(16)]);
																		
																		props.child("0", Label).run(|props| props.set_text("$140.00"));
																	});
																});
																props.child("3", Dynamic).run("p", |props| {
																	props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
																	
																	props.child("0", Label).run(|props| props.set_text("White and black"));
																});
															});
															props.child("3", Dynamic).run("div", |props| {
																props.styles(&[Style::Flex, NoStyle::Noop("flex-1"), NoStyle::Noop("items-end"), Style::JustifyBetween, NoStyle::Noop("text-sm")]);
																
																props.child("1", Dynamic).run("p", |props| {
																	props.styles(&[Style::TextColor(Color::Fg(56))]);
																	
																	props.child("0", Label).run(|props| props.set_text("Qty 1"));
																});
																props.child("3", Dynamic).run("div", |props| {
																	props.styles(&[Style::Flex]);
																	
																	props.child("1", Dynamic).run("button", |props| {
																		props.set_attribute("type", "button");
																		props.styles(&[NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("text-indigo-500")])]);
																		
																		props.child("0", Label).run(|props| props.set_text("Remove"));
																	});
																});
															});
														});
													});
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingX(16), Style::PaddingY(24), Screen::Small(&[Style::PaddingX(24)])]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::Flex, Style::JustifyBetween, NoStyle::Noop("text-base"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("1", Dynamic).run("p", |props| {
												props.child("0", Label).run(|props| props.set_text("Subtotal"));
											});
											props.child("3", Dynamic).run("p", |props| {
												props.child("0", Label).run(|props| props.set_text("$262.00"));
											});
										});
										props.child("3", Dynamic).run("p", |props| {
											props.styles(&[Style::MarginTop(2), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
											
											props.child("0", Label).run(|props| props.set_text("Shipping and taxes calculated at checkout."));
										});
										props.child("5", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(24)]);
											
											props.child("1", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), NoStyle::Noop("border-transparent"), NoStyle::Noop("bg-indigo-600"), Style::PaddingX(24), Style::PaddingY(12), NoStyle::Noop("text-base"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-white"), NoStyle::Noop("shadow-sm"), Action::Hover(&[NoStyle::Noop("bg-indigo-700")])]);
												
												props.child("0", Label).run(|props| props.set_text("Checkout"));
											});
										});
										props.child("7", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(24), Style::Flex, Style::JustifyCenter, NoStyle::Noop("text-center"), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
											
											props.child("1", Dynamic).run("p", |props| {
												props.child("0", Label).run(|props| props.set_text("or"));
												//  space 
												props.child("3", Dynamic).run("button", |props| {
													props.set_attribute("type", "button");
													props.styles(&[NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("text-indigo-500")])]);
													props.set_attribute("@click", "open = false");
													
													props.child("0", Label).run(|props| props.set_text("Continue Shopping"));
													props.child("1", Dynamic).run("span", |props| {
														props.set_attribute("aria-hidden", "true");
														
														props.child("0", Label).run(|props| props.set_text("→"));
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
